use crate::stage_c::StageC;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub struct StageBRequest {
    pub text: String,
    pub context_id: u32,
}

pub struct StageBResponse {
    pub original: String,
    pub corrected: String,
    pub context_id: u32,
}

pub struct StageB {
    tx: mpsc::Sender<StageBRequest>,
    #[allow(dead_code)]
    engine_c: Arc<Mutex<StageC>>,
}

impl StageB {
    pub fn new(response_tx: mpsc::Sender<StageBResponse>, model_dir: Option<String>) -> Self {
        let (tx, mut rx) = mpsc::channel::<StageBRequest>(100);
        let mut engine_c_raw = StageC::new();

        if let Some(dir) = model_dir {
            engine_c_raw.init_from_dir(dir);
        }

        let engine_c = Arc::new(Mutex::new(engine_c_raw));
        let engine_c_clone = engine_c.clone();

        tokio::spawn(async move {
            while let Some(req) = rx.recv().await {
                // Debounce / Delay simulado
                sleep(Duration::from_millis(150)).await;

                // Lógica de correção gramatical assíncrona (Stage B)
                let mut corrected = req.text.clone();

                // Se as regras locais do Stage B não mudarem nada, tentamos a IA (Stage C)
                if corrected == req.text {
                    if let Ok(mut engine) = engine_c_clone.lock() {
                        if let Some(ai_corrected) = engine.predict(&req.text) {
                            corrected = ai_corrected;
                        }
                    }
                }

                let _ = response_tx
                    .send(StageBResponse {
                        original: req.text,
                        corrected,
                        context_id: req.context_id,
                    })
                    .await;
            }
        });

        Self { tx, engine_c }
    }

    pub async fn request_correction(
        &self,
        text: String,
        context_id: u32,
    ) -> Result<(), mpsc::error::SendError<StageBRequest>> {
        self.tx.send(StageBRequest { text, context_id }).await
    }
}
