use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::AsyncReadExt;
use tokio::net::windows::named_pipe::ServerOptions;

pub static IS_ENABLED: AtomicBool = AtomicBool::new(true);
pub static AGGRESSIVENESS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1); // 0: Conservador, 1: Normal, 2: Agressivo
static STARTED: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize)]
pub enum IpcCommand {
    SetEnabled(bool),
    SetAggressiveness(u32),
    AddCustomWord(String),
}

pub async fn start_ipc_server(
    engine: std::sync::Arc<std::sync::Mutex<correction_engine::stage_a::StageA>>,
) {
    if STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    let pipe_name = r"\\.\pipe\ptbr_ime_pipe";

    loop {
        let server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(pipe_name);

        if let Ok(mut server) = server {
            if server.connect().await.is_ok() {
                let mut buffer = vec![0u8; 1024];
                if let Ok(n) = server.read(&mut buffer).await {
                    if let Ok(cmd) = serde_json::from_slice::<IpcCommand>(&buffer[..n]) {
                        match cmd {
                            IpcCommand::SetEnabled(enabled) => {
                                IS_ENABLED.store(enabled, Ordering::SeqCst);
                            }
                            IpcCommand::SetAggressiveness(val) => {
                                AGGRESSIVENESS.store(val, Ordering::SeqCst);
                            }
                            IpcCommand::AddCustomWord(word) => {
                                let mut engine = engine.lock().unwrap();
                                engine.load_dictionary(&[&word]);
                            }
                        }
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
