use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub const WM_TRAYICON: u32 = WM_USER + 1;
pub const ID_TRAYICON: u32 = 1;
pub const IDM_EXIT: u32 = 101;
pub const IDM_TOGGLE: u32 = 102;

/// Resolve o caminho do ícone personalizado
fn resolve_icon_path() -> std::path::PathBuf {
    let fallbacks = [
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("data").join("icon.ico")))
            .unwrap_or_default(),
        std::path::PathBuf::from(
            r"c:\System-wide AI-assisted Input Method for Portuguese (PT-BR IME)\data\icon.ico",
        ),
    ];

    for path in &fallbacks {
        if path.exists() {
            return path.clone();
        }
    }
    fallbacks[1].clone()
}

/// Carrega o ícone personalizado do dragão dourado.
unsafe fn load_custom_icon() -> HICON {
    let icon_path = resolve_icon_path();

    if icon_path.exists() {
        let path_str: Vec<u16> = icon_path
            .to_string_lossy()
            .encode_utf16()
            .chain(Some(0))
            .collect();

        let hicon = LoadImageW(
            None,
            PCWSTR(path_str.as_ptr()),
            IMAGE_ICON,
            32,
            32,
            LR_LOADFROMFILE | LR_SHARED,
        );

        if let Ok(handle) = hicon {
            if handle.0 != 0 {
                return HICON(handle.0);
            }
        }
    }

    // Fallback: ícone padrão do Windows
    LoadIconW(None, IDI_APPLICATION).unwrap_or_default()
}

pub unsafe fn create_tray_icon(hwnd: HWND) -> Result<()> {
    let mut nid = NOTIFYICONDATAW::default();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = ID_TRAYICON;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    nid.hIcon = load_custom_icon();

    let tip: Vec<u16> = "🐉 PT-BR AI IME ✓".encode_utf16().chain(Some(0)).collect();
    nid.szTip[..tip.len()].copy_from_slice(&tip);

    Shell_NotifyIconW(NIM_ADD, &nid).ok()
}

pub unsafe fn remove_tray_icon(hwnd: HWND) {
    let mut nid = NOTIFYICONDATAW::default();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = ID_TRAYICON;
    let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
}

pub unsafe fn show_context_menu(hwnd: HWND) {
    let menu = CreatePopupMenu().unwrap();

    let enabled = crate::keyboard_hook::IS_ENABLED.load(std::sync::atomic::Ordering::SeqCst);
    let toggle_text = if enabled {
        w!("⏸ Desativar Correção")
    } else {
        w!("▶ Ativar Correção")
    };

    let _ = AppendMenuW(menu, MF_STRING, IDM_TOGGLE as usize, toggle_text);
    let _ = AppendMenuW(menu, MF_SEPARATOR, 0, None);
    let _ = AppendMenuW(menu, MF_STRING, IDM_EXIT as usize, w!("✕ Sair"));

    let mut pos = POINT::default();
    let _ = GetCursorPos(&mut pos);

    SetForegroundWindow(hwnd);
    let _ = TrackPopupMenu(menu, TPM_RIGHTBUTTON, pos.x, pos.y, 0, hwnd, None);
    let _ = PostMessageW(hwnd, WM_NULL, WPARAM(0), LPARAM(0));
}
