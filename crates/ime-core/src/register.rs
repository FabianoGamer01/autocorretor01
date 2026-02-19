use crate::globals;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::{
    GetModuleFileNameW, GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
    GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
};
use windows::Win32::System::Registry::*;
use windows::Win32::UI::TextServices::*;

/// Obtém o HMODULE da DLL atual (não do processo host como regsvr32.exe).
unsafe fn get_current_dll_module() -> HINSTANCE {
    let mut hmodule = HINSTANCE::default();
    let _ = GetModuleHandleExW(
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        windows::core::PCWSTR(get_current_dll_module as *const u16),
        &mut hmodule as *mut HINSTANCE as *mut _,
    );
    hmodule
}

pub fn register_server() -> windows::core::Result<()> {
    unsafe {
        let hmodule = get_current_dll_module();
        let mut module_path = [0u16; 1024];
        let len = GetModuleFileNameW(hmodule, &mut module_path);
        let path_bytes =
            std::slice::from_raw_parts(module_path.as_ptr() as *const u8, (len as usize) * 2);

        let clsid_str = format!("{{{:?}}}", globals::CLSID_PTBR_TIP);
        let clsid_key_path = format!("CLSID\\{}", clsid_str);
        let clsid_key = HSTRING::from(&clsid_key_path);

        let mut hkey = HKEY::default();
        if RegCreateKeyW(HKEY_CLASSES_ROOT, &clsid_key, &mut hkey).is_ok() {
            let desc = HSTRING::from("PT-BR AI IME");
            let desc_wide = desc.as_wide();
            let desc_bytes =
                std::slice::from_raw_parts(desc_wide.as_ptr() as *const u8, desc_wide.len() * 2);
            let _ = RegSetValueExW(hkey, None, 0, REG_SZ, Some(desc_bytes));

            let mut hsub_key = HKEY::default();
            if RegCreateKeyW(hkey, &HSTRING::from("InprocServer32"), &mut hsub_key).is_ok() {
                let _ = RegSetValueExW(hsub_key, None, 0, REG_SZ, Some(path_bytes));

                let threading_model = HSTRING::from("Both");
                let tm_wide = threading_model.as_wide();
                let tm_bytes =
                    std::slice::from_raw_parts(tm_wide.as_ptr() as *const u8, tm_wide.len() * 2);
                let _ = RegSetValueExW(
                    hsub_key,
                    &HSTRING::from("ThreadingModel"),
                    0,
                    REG_SZ,
                    Some(tm_bytes),
                );

                let _ = RegCloseKey(hsub_key);
            }
            let _ = RegCloseKey(hkey);
        }
    }
    Ok(())
}

pub fn unregister_server() -> windows::core::Result<()> {
    unsafe {
        let clsid_str = format!("{{{:?}}}", globals::CLSID_PTBR_TIP);
        let clsid_key_path = format!("CLSID\\{}", clsid_str);
        let clsid_key = HSTRING::from(&clsid_key_path);
        let _ = RegDeleteTreeW(HKEY_CLASSES_ROOT, &clsid_key);
    }
    Ok(())
}

pub fn register_profiles() -> windows::core::Result<()> {
    unsafe {
        let input_processor_profiles: ITfInputProcessorProfiles =
            CoCreateInstance(&CLSID_TF_InputProcessorProfiles, None, CLSCTX_INPROC_SERVER)?;

        input_processor_profiles.Register(&globals::CLSID_PTBR_TIP)?;

        // Adicionar perfil de idioma PT-BR
        let description: Vec<u16> = "Português (Brasil) AI IME"
            .encode_utf16()
            .chain(Some(0))
            .collect();
        input_processor_profiles.AddLanguageProfile(
            &globals::CLSID_PTBR_TIP,
            globals::LANGID_PTBR,
            &globals::GUID_PROFILE,
            &description,
            &description, // Ícone (usando mesma string por enquanto)
            0,
        )?;
    }
    Ok(())
}

pub fn unregister_profiles() -> windows::core::Result<()> {
    unsafe {
        let input_processor_profiles: ITfInputProcessorProfiles =
            CoCreateInstance(&CLSID_TF_InputProcessorProfiles, None, CLSCTX_INPROC_SERVER)?;

        input_processor_profiles.Unregister(&globals::CLSID_PTBR_TIP)?;
    }
    Ok(())
}

pub fn register_categories() -> windows::core::Result<()> {
    unsafe {
        let category_mgr: ITfCategoryMgr =
            CoCreateInstance(&CLSID_TF_CategoryMgr, None, CLSCTX_INPROC_SERVER)?;

        // Registrar como Keyboard TIP
        category_mgr.RegisterCategory(
            &globals::CLSID_PTBR_TIP,
            &GUID_TFCAT_TIP_KEYBOARD,
            &globals::CLSID_PTBR_TIP,
        )?;

        // Registrar suporte para Display Attributes (opcional por enquanto)
        category_mgr.RegisterCategory(
            &globals::CLSID_PTBR_TIP,
            &GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER,
            &globals::CLSID_PTBR_TIP,
        )?;
    }
    Ok(())
}

pub fn unregister_categories() -> windows::core::Result<()> {
    Ok(())
}
