use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::TextServices::*;

use correction_engine::stage_a::StageA;
use std::sync::{Arc, Mutex};

#[implement(ITfTextInputProcessor, ITfTextInputProcessorEx)]
pub struct PtBrTip {
    engine: Arc<Mutex<StageA>>,
    tid: Mutex<Option<u32>>,
    thread_mgr: Mutex<Option<ITfThreadMgr>>,
}

impl PtBrTip {
    pub fn new() -> Self {
        Self {
            engine: Arc::new(Mutex::new(StageA::new())),
            tid: Mutex::new(None),
            thread_mgr: Mutex::new(None),
        }
    }
}

impl ITfTextInputProcessor_Impl for PtBrTip {
    fn Activate(&self, ptm: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        self.ActivateEx(ptm, tid, 0)
    }

    fn Deactivate(&self) -> Result<()> {
        let tid = self.tid.lock().unwrap().take();
        let thread_mgr = self.thread_mgr.lock().unwrap().take();

        if let (Some(tid), Some(tm)) = (tid, thread_mgr) {
            unsafe {
                let keystroke_mgr: ITfKeystrokeMgr = tm.cast()?;
                keystroke_mgr.UnadviseKeyEventSink(tid)?;
            }
        }

        // Desativar fallback hook ao desativar o TIP
        crate::fallback::FallbackManager::stop_global_hook();

        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for PtBrTip {
    fn ActivateEx(&self, ptm: Option<&ITfThreadMgr>, tid: u32, _dwflags: u32) -> Result<()> {
        // Iniciar Servidor IPC em thread separada (sem Tokio runtime)
        // Usa uma thread dedicada com um Tokio runtime próprio.
        {
            let engine_clone = self.engine.clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();
                if let Ok(rt) = rt {
                    rt.block_on(crate::ipc::start_ipc_server(engine_clone));
                }
            });
        }

        // Carregar dicionário usando caminho dinâmico
        let mut engine = self.engine.lock().unwrap();
        let dict_path = crate::globals::resolve_dict_path();

        // Log para debug
        if dict_path.exists() {
            if let Ok(words) =
                correction_engine::dict_loader::load_from_file(dict_path.to_str().unwrap_or(""))
            {
                engine.load_dictionary_strings(&words);
            }
        }
        drop(engine);

        // Inicializar o Fallback com o engine compartilhado
        // (para apps que não suportam TSF — jogos, apps Java, etc.)
        crate::fallback::FallbackManager::init(self.engine.clone());

        // Registrar Sink de Teclado TSF
        if let Some(ptm) = ptm {
            unsafe {
                let keystroke_mgr: ITfKeystrokeMgr = ptm.cast()?;
                let sink: ITfKeyEventSink =
                    crate::key_event::PtBrKeyEventSink::new(self.engine.clone(), tid).into();

                keystroke_mgr.AdviseKeyEventSink(tid, &sink, TRUE)?;

                // Guardar para Deactivate
                *self.tid.lock().unwrap() = Some(tid);
                *self.thread_mgr.lock().unwrap() = Some(ptm.clone());
            }
        } else {
            // Sem contexto TSF: ativar fallback global automaticamente
            let _ = crate::fallback::FallbackManager::start_global_hook();
        }

        Ok(())
    }
}
