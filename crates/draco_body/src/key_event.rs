use crate::composition::CompositionManager;
use draco_brain::stage_a::StageA;
use std::sync::{Arc, Mutex};
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::TextServices::*;

#[implement(ITfKeyEventSink)]
pub struct PtBrKeyEventSink {
    engine: Arc<Mutex<StageA>>,
    composition: Arc<Mutex<CompositionManager>>,
    _client_id: u32,
}

impl PtBrKeyEventSink {
    pub fn new(engine: Arc<Mutex<StageA>>, client_id: u32) -> Self {
        Self {
            engine,
            composition: Arc::new(Mutex::new(CompositionManager::new())),
            _client_id: client_id,
        }
    }
}

impl ITfKeyEventSink_Impl for PtBrKeyEventSink {
    fn OnSetFocus(&self, pfid: BOOL) -> Result<()> {
        let _ = pfid;
        Ok(())
    }

    fn OnTestKeyDown(
        &self,
        _pic: Option<&ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        let vk = wparam.0 as u16;
        // Só interceptamos o Espaço para fazer a correção.
        // As letras NÃO são interceptadas — o app nativo lida com elas.
        if vk == VK_SPACE.0 {
            let comp = self.composition.lock().unwrap();
            if !comp.get_buffer().is_empty() {
                return Ok(TRUE); // Intercepta Espaço apenas se tiver buffer
            }
        }
        Ok(FALSE) // Deixa todas as outras teclas passarem
    }

    fn OnTestKeyUp(
        &self,
        _pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(FALSE)
    }

    fn OnKeyDown(&self, pic: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        if !crate::ipc::IS_ENABLED.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(FALSE);
        }

        let vk = wparam.0 as u16;
        let mut comp = self.composition.lock().unwrap();

        if vk >= V_K_A && vk <= V_K_Z {
            // Letra: adiciona ao buffer interno MAS NÃO intercepta a tecla
            // O app continua recebendo a letra normalmente
            let c = (b'a' + (vk - V_K_A) as u8) as char;
            comp.add_char(c);
            return Ok(FALSE); // ← NÃO engole a tecla!
        } else if vk == VK_BACK.0 {
            comp.backspace();
            return Ok(FALSE);
        } else if vk == VK_SPACE.0 {
            let word = comp.get_buffer().to_string();
            if !word.is_empty() {
                let agg = crate::ipc::AGGRESSIVENESS.load(std::sync::atomic::Ordering::SeqCst);
                let engine = self.engine.lock().unwrap();
                let corrected = engine.correct(&word, agg);
                drop(engine);
                comp.clear();

                if corrected != word {
                    // A palavra foi corrigida!
                    // Estratégia: apagar a palavra digitada via backspaces + digitar a palavra corrigida + espaço
                    if let Some(_pic) = pic {
                        // No contexto TSF, usamos SendInput (mais confiável que EditSession)
                        unsafe {
                            crate::fallback::FallbackManager::send_backspaces_public(word.len());
                            crate::fallback::FallbackManager::send_text(&corrected);
                            // O Espaço será inserido pelo app (retornamos FALSE)
                        }
                    } else {
                        unsafe {
                            crate::fallback::FallbackManager::send_backspaces_public(word.len());
                            crate::fallback::FallbackManager::send_text(&corrected);
                        }
                    }
                    return Ok(FALSE); // Deixa o espaço passar normalmente
                } else {
                    comp.clear();
                    return Ok(FALSE);
                }
            }
            return Ok(FALSE);
        } else {
            // Qualquer outra tecla: limpa buffer (Enter, Tab, setas, etc.)
            if vk != VK_SHIFT.0 && vk != VK_CONTROL.0 && vk != VK_MENU.0 && vk != VK_CAPITAL.0 {
                comp.clear();
            }
        }

        Ok(FALSE)
    }

    fn OnKeyUp(&self, _pic: Option<&ITfContext>, _wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        Ok(FALSE)
    }

    fn OnPreservedKey(&self, _pic: Option<&ITfContext>, _rguid: *const GUID) -> Result<BOOL> {
        Ok(FALSE)
    }
}

const V_K_A: u16 = 0x41;
const V_K_Z: u16 = 0x5A;
