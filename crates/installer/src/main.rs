use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("PT-BR AI IME - Instalador do Sistema");
    println!("------------------------------------");

    // 1. Localizar a DLL (assumindo que está no mesmo diretório ou no target/debug)
    let mut dll_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    dll_path.pop(); // Sair da pasta do executável

    // Tenta caminhos comuns em desenvolvimento
    let candidates = [
        dll_path.join("ime_core.dll"),
        dll_path.join("../ime_core.dll"),
        dll_path.join("../../target/debug/ime_core.dll"),
        dll_path.join("../../target/release/ime_core.dll"),
    ];

    let mut found_path = None;
    for path in &candidates {
        if path.exists() {
            found_path = Some(path.to_owned());
            break;
        }
    }

    let target_dll = match found_path {
        Some(p) => p,
        None => {
            eprintln!("ERRO: Não foi possível encontrar 'ime_core.dll'!");
            println!("Procurei em: {:?}", candidates);
            return;
        }
    };

    println!("Localizado: {:?}", target_dll);

    // 2. Registrar a DLL via regsvr32
    println!("Registrando componente COM e TSF...");
    let status = Command::new("regsvr32")
        .arg("/s") // Modo silencioso
        .arg(&target_dll)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("SUCESSO: O IME foi registrado no sistema!");
            println!("Agora você pode ativá-lo nas 'Configurações de Idioma' do Windows.");
        }
        _ => {
            eprintln!(
                "ERRO: Falha ao registrar a DLL. Certifique-se de executar como ADMINISTRADOR."
            );
        }
    }
}
