#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows::core::{Interface, HSTRING, PCWSTR};
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitialize, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
};
use windows::Win32::UI::Shell::{
    IShellLinkW, SHGetFolderPathW, ShellLink, CSIDL_DESKTOPDIRECTORY, CSIDL_FLAG_CREATE,
    CSIDL_PROGRAM_FILES, CSIDL_STARTUP,
};

// Incorporar os binários compilados diretamente no executável final
// NOTA: Os arquivos devem existir em target/release antes de compilar este crate.
const TRAY_APP_BYTES: &[u8] = include_bytes!("../../../target/release/draco_head.exe");
const IME_DLL_BYTES: &[u8] = include_bytes!("../../../target/release/draco_body.dll");
const DIRECTML_BYTES: &[u8] = include_bytes!("../../../target/release/DirectML.dll");

fn main() {
    // 1. Verificar privilégios de administrador
    if !is_admin() {
        // Se não for admin, tentar reiniciar com privilégios elevados
        println!("Solicitando permissões de administrador...");
        run_as_admin();
        return;
    }

    println!("Iniciando instalação do Autocorretor PT-BR...");

    // 2. Definir diretórios de instalação
    let program_files = unsafe { get_special_folder(CSIDL_PROGRAM_FILES) }
        .expect("Não foi possível localizar Program Files");
    let install_dir = PathBuf::from(program_files).join("AutocorretorPTBR");

    if !install_dir.exists() {
        fs::create_dir_all(&install_dir).expect("Falha ao criar diretório de instalação");
    }

    let tray_path = install_dir.join("draco_head.exe");
    let dll_path = install_dir.join("draco_body.dll");

    // 3. Parar processos existentes
    kill_process("draco_head.exe");

    // 4. Copiar arquivos
    println!("Copiando arquivos...");
    fs::write(&tray_path, TRAY_APP_BYTES).expect("Falha ao escrever draco_head.exe");
    fs::write(&dll_path, IME_DLL_BYTES).expect("Falha ao escrever draco_body.dll");

    let directml_path = install_dir.join("DirectML.dll");
    fs::write(&directml_path, DIRECTML_BYTES).expect("Falha ao escrever DirectML.dll");

    // 5. Registrar DLL
    println!("Registrando IME...");
    let status = Command::new("regsvr32")
        .arg("/s")
        .arg(&dll_path)
        .status()
        .expect("Falha ao executar regsvr32");

    if !status.success() {
        eprintln!(
            "ERRO: Falha ao registrar a DLL. Código de saída: {:?}",
            status.code()
        );
    } else {
        println!("IME registrado com sucesso!");
    }

    // 6. Criar Atalhos
    unsafe {
        let _ = CoInitialize(None);

        // Atalho na Área de Trabalho
        if let Some(desktop) = get_special_folder(CSIDL_DESKTOPDIRECTORY) {
            let lnk_path = PathBuf::from(desktop).join("Autocorretor PT-BR.lnk");
            create_shortcut(
                &tray_path,
                &lnk_path,
                "Inicia o Autocorretor com suporte a IA",
            );
            println!("Atalho criado na Área de Trabalho.");
        }

        // Atalho no Menu Iniciar (Startup)
        if let Some(startup) = get_special_folder(CSIDL_STARTUP) {
            let lnk_path = PathBuf::from(startup).join("Autocorretor.lnk");
            create_shortcut(&tray_path, &lnk_path, "Autostart do Autocorretor PT-BR");
            println!("Atalho criado na Inicialização.");
        }

        CoUninitialize();
    }

    // 7. Iniciar a aplicação
    println!("Iniciando aplicação...");
    Command::new(&tray_path).spawn().ok();

    println!("Instalação concluída com sucesso!");
}

fn is_admin() -> bool {
    // Uma verificação simples: tentar escrever em C:\Windows\System32 (ou testar token)
    // Aqui vamos usar um truque comum: tentar criar um diretório temporário em system32 é arriscado.
    // Melhor usar a API IsUserAnAdmin se disponível, mas podemos inferir tentando algo restrito.
    // Ou simplesmente confiar que o runas fará o trabalho.
    // Para simplificar, vamos assumir que se falhar a criação do dir em Program Files, não somos admin.
    // Mas o ideal é o manifesto. Vamos usar o truque do `net session`.
    let output = Command::new("net").arg("session").output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

fn run_as_admin() {
    let current_exe = env::current_exe().unwrap();

    // Usar PowerShell para elevar, é mais simples em Rust puro sem crates complexos de winapi shell execute
    Command::new("powershell")
        .arg("-Command")
        .arg(format!("Start-Process '{:?}' -Verb RunAs", current_exe))
        .spawn()
        .ok();
}

fn kill_process(name: &str) {
    Command::new("taskkill")
        .args(&["/F", "/IM", name])
        .output()
        .ok();
}

unsafe fn get_special_folder(csidl: u32) -> Option<String> {
    let mut path = [0u16; MAX_PATH as usize];
    if SHGetFolderPathW(
        None,
        csidl as i32 | CSIDL_FLAG_CREATE as i32,
        None,
        0,
        &mut path,
    )
    .is_ok()
    {
        let len = path.iter().position(|&c| c == 0).unwrap_or(path.len());
        Some(String::from_utf16_lossy(&path[..len]))
    } else {
        None
    }
}

unsafe fn create_shortcut(target: &Path, lnk_path: &Path, description: &str) {
    let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
        .unwrap_or_else(|_| panic!("Falha ao criar ShellLink"));

    let target_hstring = HSTRING::from(target.to_str().unwrap());
    shell_link.SetPath(&target_hstring).ok();

    let work_dir = target.parent().unwrap();
    let work_dir_hstring = HSTRING::from(work_dir.to_str().unwrap());
    shell_link.SetWorkingDirectory(&work_dir_hstring).ok();

    let desc_hstring = HSTRING::from(description);
    shell_link.SetDescription(&desc_hstring).ok();

    let persist_file: IPersistFile = shell_link.cast().unwrap();
    let lnk_hstring = HSTRING::from(lnk_path.to_str().unwrap());
    persist_file.Save(&lnk_hstring, true).ok();
}
