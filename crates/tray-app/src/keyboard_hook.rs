use correction_engine::stage_a::StageA;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// Flag global: correção ativada/desativada
pub static IS_ENABLED: AtomicBool = AtomicBool::new(true);

/// Nível de agressividade
pub static AGGRESSIVENESS: AtomicU32 = AtomicU32::new(1);

/// Flag para ignorar eventos durante correção ativa
static IS_CORRECTING: AtomicBool = AtomicBool::new(false);

/// Handle do hook
static HOOK_HANDLE: AtomicIsize = AtomicIsize::new(0);

/// Engine de correção (thread-safe)
static ENGINE: OnceLock<Arc<Mutex<StageA>>> = OnceLock::new();

/// Buffer de composição (thread-safe)
static BUFFER: OnceLock<Arc<Mutex<String>>> = OnceLock::new();

/// Contador de caracteres reais digitados (para backspaces corretos)
static CHAR_COUNT: OnceLock<Arc<Mutex<usize>>> = OnceLock::new();

/// Flag LLKHF_INJECTED
const LLKHF_INJECTED: u32 = 0x00000010;

/// Inicializa o engine de correção.
pub fn init_engine(engine: StageA) {
    let _ = ENGINE.set(Arc::new(Mutex::new(engine)));
    let _ = BUFFER.set(Arc::new(Mutex::new(String::new())));
    let _ = CHAR_COUNT.set(Arc::new(Mutex::new(0)));
}

/// Instala o hook global de teclado.
pub fn start_hook() -> windows::core::Result<()> {
    if HOOK_HANDLE.load(Ordering::SeqCst) != 0 {
        return Ok(());
    }

    unsafe {
        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            HINSTANCE(0),
            0,
        )?;
        HOOK_HANDLE.store(hook.0, Ordering::SeqCst);
    }
    Ok(())
}

/// Remove o hook global.
pub fn stop_hook() {
    let handle = HOOK_HANDLE.swap(0, Ordering::SeqCst);
    if handle != 0 {
        unsafe {
            let _ = UnhookWindowsHookEx(HHOOK(handle));
        }
    }
}

/// Envia N backspaces via SendInput.
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

/// Injeta texto via SendInput (Unicode).
unsafe fn send_text(text: &str) {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let mut inputs = Vec::with_capacity(utf16.len() * 2);

    for &c in &utf16 {
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

/// Executa a correção em uma THREAD SEPARADA para não bloquear o hook.
fn spawn_correction(word: String, char_count: usize) {
    let engine_arc = match ENGINE.get() {
        Some(e) => Arc::clone(e),
        None => return,
    };

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(30));

        let agg = AGGRESSIVENESS.load(Ordering::SeqCst);
        let corrected = if let Ok(engine) = engine_arc.lock() {
            engine.correct(&word, agg)
        } else {
            return;
        };

        if corrected != word {
            IS_CORRECTING.store(true, Ordering::SeqCst);

            unsafe {
                // Usar char_count para apagar o número correto de caracteres
                // (pode ser diferente de word.len() se houve dead keys/acentos)
                send_backspaces(char_count + 1); // +1 para o espaço
                std::thread::sleep(std::time::Duration::from_millis(20));
                send_text(&corrected);
                send_text(" ");
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
            IS_CORRECTING.store(false, Ordering::SeqCst);
        }
    });
}

/// Verifica se um VK code é uma tecla que QUEBRA a palavra (cursor, enter, etc.)
fn is_word_boundary_key(vk: u16) -> bool {
    matches!(
        vk,
        0x0D   // VK_RETURN (Enter)
        | 0x09 // VK_TAB
        | 0x1B // VK_ESCAPE
        | 0x25 // VK_LEFT
        | 0x26 // VK_UP
        | 0x27 // VK_RIGHT
        | 0x28 // VK_DOWN
        | 0x24 // VK_HOME
        | 0x23 // VK_END
        | 0x21 // VK_PRIOR (Page Up)
        | 0x22 // VK_NEXT (Page Down)
        | 0x2E // VK_DELETE
        | 0x2D // VK_INSERT
    )
}

/// Verifica se um VK code é um modificador ou tecla que NÃO afeta a palavra.
/// Dead keys, Shift, Ctrl, Alt, CapsLock, etc.
fn is_ignorable_key(vk: u16) -> bool {
    matches!(
        vk,
        0x10   // VK_SHIFT
        | 0x11 // VK_CONTROL
        | 0x12 // VK_MENU (Alt)
        | 0x14 // VK_CAPITAL (CapsLock)
        | 0xA0 // VK_LSHIFT
        | 0xA1 // VK_RSHIFT
        | 0xA2 // VK_LCONTROL
        | 0xA3 // VK_RCONTROL
        | 0xA4 // VK_LMENU
        | 0xA5 // VK_RMENU
        | 0x5B // VK_LWIN
        | 0x5C // VK_RWIN
        | 0x5D // VK_APPS (Menu key)
        | 0x90 // VK_NUMLOCK
        | 0x91 // VK_SCROLL
        | 0x2C // VK_SNAPSHOT (Print Screen)
        | 0x13 // VK_PAUSE
        // Dead keys no ABNT2 (acento agudo, til, circunflexo, crase)
        // Estes NÃO devem limpar o buffer!
        | 0xDE // VK_OEM_7 (acento agudo ´ / crase ` no ABNT2)
        | 0xC0 // VK_OEM_3 (aspas no ABNT2, pode ser dead key)
        | 0xDB // VK_OEM_4  ([ { / dead keys)
        | 0xDD // VK_OEM_6 (] } / dead keys)
        // F-keys
        | 0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 // F1-F6
        | 0x76 | 0x77 | 0x78 | 0x79 | 0x7A | 0x7B // F7-F12
    )
}

/// Callback do Low-Level Keyboard Hook.
/// IMPORTANTE: Retorna o mais rápido possível.
unsafe extern "system" fn low_level_keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let hook_handle = HHOOK(HOOK_HANDLE.load(Ordering::SeqCst));

    if code >= 0 && wparam.0 == 0x0100 {
        // WM_KEYDOWN
        let kbd = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

        // Ignorar eventos injetados
        if kbd.flags.0 & LLKHF_INJECTED != 0 {
            return CallNextHookEx(hook_handle, code, wparam, lparam);
        }

        // Ignorar durante correção ativa
        if IS_CORRECTING.load(Ordering::SeqCst) {
            return CallNextHookEx(hook_handle, code, wparam, lparam);
        }

        // Só processar se correção estiver habilitada
        if IS_ENABLED.load(Ordering::SeqCst) {
            let vk = kbd.vkCode as u16;

            const VK_A: u16 = 0x41;
            const VK_Z: u16 = 0x5A;

            if let (Some(buffer_lock), Some(count_lock)) = (BUFFER.get(), CHAR_COUNT.get()) {
                if vk >= VK_A && vk <= VK_Z {
                    // ==== LETRA A-Z ====
                    // Adiciona ao buffer como minúscula
                    let c = (b'a' + (vk - VK_A) as u8) as char;
                    if let Ok(mut buf) = buffer_lock.lock() {
                        buf.push(c);
                    }
                    if let Ok(mut cnt) = count_lock.lock() {
                        *cnt += 1;
                    }
                } else if vk == 0xBA {
                    // ==== ç no ABNT2 (VK_OEM_1 = 0xBA) ====
                    // No teclado ABNT2, a tecla ç tem VK code 0xBA
                    // Não adicionamos ao buffer (o motor de correção trabalha sem ç)
                    // Mas contamos como caractere para backspace correto
                    if let Ok(mut cnt) = count_lock.lock() {
                        *cnt += 1;
                    }
                    // Não adicionamos ao buffer de letras — o engine vai corrigir
                    // baseado nas letras ASCII e o TypoModel/fonético cuida do resto
                } else if vk == VK_BACK.0 {
                    // ==== BACKSPACE ====
                    if let Ok(mut buf) = buffer_lock.lock() {
                        buf.pop();
                    }
                    if let Ok(mut cnt) = count_lock.lock() {
                        *cnt = cnt.saturating_sub(1);
                    }
                } else if vk == VK_SPACE.0 {
                    // ==== ESPAÇO: trigger de correção ====
                    let word;
                    let char_count;
                    {
                        word = if let Ok(mut buf) = buffer_lock.lock() {
                            let w = buf.clone();
                            buf.clear();
                            w
                        } else {
                            String::new()
                        };
                        char_count = if let Ok(mut cnt) = count_lock.lock() {
                            let c = *cnt;
                            *cnt = 0;
                            c
                        } else {
                            word.len()
                        };
                    }

                    if !word.is_empty() {
                        spawn_correction(word, char_count);
                    }
                } else if is_word_boundary_key(vk) {
                    // ==== TECLAS DE FRONTEIRA: limpam o buffer ====
                    // Enter, Tab, setas, Home, End, etc.
                    if let Ok(mut buf) = buffer_lock.lock() {
                        buf.clear();
                    }
                    if let Ok(mut cnt) = count_lock.lock() {
                        *cnt = 0;
                    }
                } else if is_ignorable_key(vk) {
                    // ==== TECLAS IGNORÁVEIS: dead keys, modificadores, F-keys ====
                    // NÃO limpar o buffer! Dead keys (´, ~, ^, `) fazem parte da digitação
                    // mas contam como caractere (para backspace)
                    // Verificar se é uma dead key que produz caractere visível
                    if vk == 0xDE || vk == 0xDB || vk == 0xDD || vk == 0xC0 {
                        // Dead keys — contam como parte do caractere seguinte
                        // O caractere final (é, ã, ô) conta como 1, mas a dead key + letra = 1 char visível
                        // Não incrementamos o contador aqui — o resultado conta como 1 char com a letra
                    }
                    // Não faz nada ao buffer
                } else if (0x30..=0x39).contains(&vk) {
                    // ==== NÚMEROS (0-9) ====
                    // Limpa buffer (números quebram palavras)
                    if let Ok(mut buf) = buffer_lock.lock() {
                        buf.clear();
                    }
                    if let Ok(mut cnt) = count_lock.lock() {
                        *cnt = 0;
                    }
                } else {
                    // ==== OUTRAS TECLAS ====
                    // Teclas desconhecidas: NÃO limpar o buffer por segurança
                    // Podem ser OEM keys, teclas especiais, etc.
                    // Só não adicionamos ao buffer de letras
                }
            }
        }
    }

    CallNextHookEx(hook_handle, code, wparam, lparam)
}
