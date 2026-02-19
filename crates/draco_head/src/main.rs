#![windows_subsystem = "windows"]

mod keyboard_hook;
mod tray;

use tray::*;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() -> Result<()> {
    // 1. Inicializar engine de correção
    let mut engine = draco_brain::stage_a::StageA::new();

    // 2. Carregar dados de FREQUÊNCIA primeiro (para que o dicionário já tenha os ranks)
    let freq_path = resolve_freq_path();
    if freq_path.exists() {
        if let Ok(entries) = draco_brain::dict_loader::load_frequency_file(&freq_path) {
            eprintln!(
                "[IME] Frequências carregadas: {} palavras de {:?}",
                entries.len(),
                freq_path
            );
            engine.load_frequency_data(&entries);
        }
    }

    // 3. Carregar dicionário
    let dict_path = resolve_dict_path();
    if dict_path.exists() {
        if let Ok(words) = draco_brain::dict_loader::load_from_file(&dict_path) {
            engine.load_dictionary_strings(&words);
            eprintln!(
                "[IME] Dicionário carregado: {} palavras de {:?}",
                words.len(),
                dict_path
            );
        } else {
            eprintln!("[IME] Erro ao carregar dicionário de {:?}", dict_path);
        }
    } else {
        eprintln!("[IME] Dicionário não encontrado em {:?}", dict_path);
    }

    // 4. Inicializar o engine no hook de teclado
    keyboard_hook::init_engine(engine);

    // 5. Instalar hook global de teclado
    keyboard_hook::start_hook()?;
    eprintln!("[IME] Hook de teclado global instalado!");

    // 4. Criar janela oculta + ícone na system tray
    unsafe {
        let instance = HINSTANCE(GetModuleHandleW(None)?.0);
        let window_class = w!("PtBrImeTrayClass");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            lpszClassName: window_class,
            ..Default::default()
        };

        if RegisterClassW(&wc) == 0 {
            return Err(Error::from(E_FAIL));
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("PT-BR AI IME"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        if hwnd.0 == 0 {
            return Err(Error::from(E_FAIL));
        }

        create_tray_icon(hwnd)?;
        eprintln!("[IME] Ícone na system tray criado. App rodando em segundo plano.");

        // 6. Message Loop (mantém o app vivo e o hook funcionando)
        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }

        // 7. Cleanup
        keyboard_hook::stop_hook();
        remove_tray_icon(hwnd);
    }
    Ok(())
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            if lparam.0 as u32 == WM_RBUTTONUP {
                show_context_menu(hwnd);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let id = wparam.0 as u32;
            match id {
                IDM_EXIT => {
                    keyboard_hook::stop_hook();
                    PostQuitMessage(0);
                }
                IDM_TOGGLE => {
                    let currently_enabled =
                        keyboard_hook::IS_ENABLED.load(std::sync::atomic::Ordering::SeqCst);
                    keyboard_hook::IS_ENABLED
                        .store(!currently_enabled, std::sync::atomic::Ordering::SeqCst);
                    eprintln!(
                        "[IME] Correção {}",
                        if !currently_enabled {
                            "ATIVADA"
                        } else {
                            "DESATIVADA"
                        }
                    );
                }
                _ => {}
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            keyboard_hook::stop_hook();
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// Resolve o caminho do dicionário
fn resolve_dict_path() -> std::path::PathBuf {
    let fallbacks = [
        // Relativo ao executável
        std::env::current_exe()
            .ok()
            .and_then(|p| {
                p.parent()
                    .map(|d| d.join("data").join("dictionary_pt_br.txt"))
            })
            .unwrap_or_default(),
        // Caminho absoluto do projeto
        std::path::PathBuf::from(
            r"c:\System-wide AI-assisted Input Method for Portuguese (PT-BR IME)\data\dictionary_pt_br.txt",
        ),
    ];

    for path in &fallbacks {
        if path.exists() {
            return path.clone();
        }
    }

    // Último recurso
    fallbacks[1].clone()
}

/// Resolve o caminho do arquivo de frequência
fn resolve_freq_path() -> std::path::PathBuf {
    let fallbacks = [
        std::env::current_exe()
            .ok()
            .and_then(|p| {
                p.parent()
                    .map(|d| d.join("data").join("frequency_pt_br.txt"))
            })
            .unwrap_or_default(),
        std::path::PathBuf::from(
            r"c:\System-wide AI-assisted Input Method for Portuguese (PT-BR IME)\data\frequency_pt_br.txt",
        ),
    ];

    for path in &fallbacks {
        if path.exists() {
            return path.clone();
        }
    }

    fallbacks[1].clone()
}
