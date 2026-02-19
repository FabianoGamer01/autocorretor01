use std::path::PathBuf;
use windows::core::GUID;

// {B2478B55-CAA7-4846-9704-38870CE71E0D}
pub const CLSID_PTBR_TIP: GUID = GUID::from_u128(0xB2478B55_CAA7_4846_9704_38870CE71E0D);

// {D0E1F203-1192-4392-83B3-2B30193E83B3}
pub const GUID_PROFILE: GUID = GUID::from_u128(0xD0E1F203_1192_4392_83B3_2B30193E83B3);

pub const LANGID_PTBR: u16 = 0x0416; // Português (Brasil)

/// Resolve o caminho do dicionário dinamicamente, baseado na localização da DLL.
/// Procura `data/dictionary_pt_br.txt` relativo ao diretório da DLL.
/// Se não encontrar, tenta caminhos fallback conhecidos.
pub fn resolve_dict_path() -> PathBuf {
    // Primeiro: tenta relativo à DLL
    if let Some(dll_dir) = get_dll_directory() {
        let relative = dll_dir
            .join("..")
            .join("..")
            .join("data")
            .join("dictionary_pt_br.txt");
        if relative.exists() {
            return relative;
        }
        // Tenta diretamente no diretório pai
        let parent = dll_dir.join("data").join("dictionary_pt_br.txt");
        if parent.exists() {
            return parent;
        }
    }

    // Fallback: caminhos conhecidos
    let fallbacks = [
        r"c:\System-wide AI-assisted Input Method for Portuguese (PT-BR IME)\data\dictionary_pt_br.txt",
        r"c:\Users\fabia\OneDrive\Documentos\System-wide AI-assisted Input Method for Portuguese (PT-BR IME)\data\dictionary_pt_br.txt",
    ];

    for path in &fallbacks {
        let p = PathBuf::from(path);
        if p.exists() {
            return p;
        }
    }

    // Último recurso
    PathBuf::from(fallbacks[0])
}

/// Resolve o caminho do diretório de modelos ONNX dinamicamente.
pub fn resolve_model_dir() -> PathBuf {
    if let Some(dll_dir) = get_dll_directory() {
        let relative = dll_dir.join("..").join("..").join("data").join("models");
        if relative.exists() {
            return relative;
        }
    }

    let fallbacks = [
        r"c:\System-wide AI-assisted Input Method for Portuguese (PT-BR IME)\data\models",
        r"c:\Users\fabia\OneDrive\Documentos\System-wide AI-assisted Input Method for Portuguese (PT-BR IME)\data\models",
    ];

    for path in &fallbacks {
        let p = PathBuf::from(path);
        if p.exists() {
            return p;
        }
    }

    PathBuf::from(fallbacks[0])
}

/// Obtém o diretório da DLL em execução.
fn get_dll_directory() -> Option<PathBuf> {
    use windows::Win32::Foundation::HINSTANCE;
    use windows::Win32::System::LibraryLoader::{
        GetModuleFileNameW, GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
        GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
    };

    unsafe {
        let mut hmodule = HINSTANCE::default();
        let _ = GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            windows::core::PCWSTR(get_dll_directory as *const u16),
            &mut hmodule as *mut HINSTANCE as *mut _,
        );

        let mut path = [0u16; 1024];
        let len = GetModuleFileNameW(hmodule, &mut path);
        if len > 0 {
            let path_str = String::from_utf16_lossy(&path[..len as usize]);
            let p = PathBuf::from(path_str);
            p.parent().map(|p| p.to_path_buf())
        } else {
            None
        }
    }
}
