use draco_brain::stage_a::StageA;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

static IS_FALLBACK_ACTIVE: AtomicBool = AtomicBool::new(false);
static mut HOOK_HANDLE: HHOOK = HHOOK(0);

/// Estado global compartilhado com o hook de teclado.
/// Inicializado pelo TIP quando o fallback é ativado.
static FALLBACK_ENGINE: OnceLock<Arc<Mutex<StageA>>> = OnceLock::new();

/// Buffer de composição do fallback (palavras sendo digitadas fora do TSF).
static FALLBACK_BUFFER: OnceLock<Arc<Mutex<String>>> = OnceLock::new();

pub struct FallbackManager;

impl FallbackManager {
    pub fn is_active() -> bool {
        IS_FALLBACK_ACTIVE.load(Ordering::SeqCst)
    }

    /// Inicializa o estado compartilhado do fallback com o engine de correção.
    /// Deve ser chamado pelo TIP antes de ativar o hook.
    pub fn init(engine: Arc<Mutex<StageA>>) {
        let _ = FALLBACK_ENGINE.set(engine);
        let _ = FALLBACK_BUFFER.set(Arc::new(Mutex::new(String::new())));
    }

    /// Ativa o hook global (Low Level Keyboard Hook).
    pub fn start_global_hook() -> windows::core::Result<()> {
        if Self::is_active() {
            return Ok(());
        }

        unsafe {
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(Self::low_level_keyboard_proc),
                HINSTANCE(0),
                0,
            )?;
            HOOK_HANDLE = hook;
            IS_FALLBACK_ACTIVE.store(true, Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn stop_global_hook() {
        if !Self::is_active() {
            return;
        }

        unsafe {
            if HOOK_HANDLE.0 != 0 {
                let _ = UnhookWindowsHookEx(HOOK_HANDLE);
                HOOK_HANDLE = HHOOK(0);
            }
            IS_FALLBACK_ACTIVE.store(false, Ordering::SeqCst);
        }
    }

    /// Injeta uma string no aplicativo ativo simulando eventos de teclado Unicode.
    pub unsafe fn send_text(text: &str) {
        let utf16: Vec<u16> = text.encode_utf16().collect();
        let mut inputs = Vec::with_capacity(utf16.len() * 2);

        for &c in &utf16 {
            // Key down
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: c,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
            // Key up
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: c,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
        }

        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }

    /// Envia N backspaces para apagar caracteres. (API pública para key_event.rs)
    pub unsafe fn send_backspaces_public(count: usize) {
        Self::send_backspaces(count);
    }

    /// Envia N backspaces para apagar caracteres.
    unsafe fn send_backspaces(count: usize) {
        let mut inputs = Vec::with_capacity(count * 2);
        for _ in 0..count {
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
        }
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }

    /// Callback do hook de teclado global.
    /// Intercepta teclas, acumula no buffer e corrige ao pressionar Espaço.
    unsafe extern "system" fn low_level_keyboard_proc(
        code: i32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        // Só processa eventos de key-down (WM_KEYDOWN = 0x0100)
        if code >= 0 && wparam.0 == 0x0100 {
            // Só processa se o IME estiver habilitado
            if crate::ipc::IS_ENABLED.load(Ordering::SeqCst) {
                let kbd = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
                let vk = kbd.vkCode as u16;

                // Constantes de teclas virtuais
                const VK_A: u16 = 0x41;
                const VK_Z: u16 = 0x5A;

                if let (Some(engine_arc), Some(buffer_arc)) =
                    (FALLBACK_ENGINE.get(), FALLBACK_BUFFER.get())
                {
                    if vk >= VK_A && vk <= VK_Z {
                        // Tecla de letra: adiciona ao buffer
                        let c = (b'a' + (vk - VK_A) as u8) as char;
                        if let Ok(mut buf) = buffer_arc.lock() {
                            buf.push(c);
                        }
                    } else if vk == VK_BACK.0 {
                        // Backspace: remove último char do buffer
                        if let Ok(mut buf) = buffer_arc.lock() {
                            buf.pop();
                        }
                    } else if vk == VK_SPACE.0 {
                        // Espaço: corrige a palavra acumulada
                        let word = {
                            if let Ok(mut buf) = buffer_arc.lock() {
                                let w = buf.clone();
                                buf.clear();
                                w
                            } else {
                                String::new()
                            }
                        };

                        if !word.is_empty() {
                            let agg = crate::ipc::AGGRESSIVENESS.load(Ordering::SeqCst);

                            let corrected = if let Ok(engine) = engine_arc.lock() {
                                engine.correct(&word, agg)
                            } else {
                                word.clone()
                            };

                            // Se a palavra foi corrigida, substituir via SendInput:
                            // apaga os chars digitados + envia a correção
                            if corrected != word {
                                // Apaga a palavra digitada (word.len() backspaces)
                                Self::send_backspaces(word.len());
                                // Injeta a palavra corrigida
                                Self::send_text(&corrected);
                            }
                        }
                    } else {
                        // Qualquer outra tecla (Enter, Esc, setas, etc.) limpa o buffer
                        if vk != VK_SHIFT.0
                            && vk != VK_CONTROL.0
                            && vk != VK_MENU.0
                            && vk != VK_CAPITAL.0
                        {
                            if let Ok(mut buf) = buffer_arc.lock() {
                                buf.clear();
                            }
                        }
                    }
                }
            }
        }

        CallNextHookEx(HOOK_HANDLE, code, wparam, lparam)
    }
}
