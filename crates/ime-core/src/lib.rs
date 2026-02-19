use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;

mod composition;
mod edit_session;
mod factory;
mod fallback;
mod globals;
mod ipc;
mod key_event;
mod register;
mod tip;

use factory::PtBrTipFactory;

#[no_mangle]
pub extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut std::ffi::c_void,
) -> HRESULT {
    if rclsid.is_null() || riid.is_null() || ppv.is_null() {
        return E_INVALIDARG;
    }

    unsafe {
        if *rclsid != globals::CLSID_PTBR_TIP {
            return CLASS_E_CLASSNOTAVAILABLE;
        }

        let factory: IClassFactory = PtBrTipFactory.into();
        factory.query(riid, ppv)
    }
}

#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    // Para simplificar agora, sempre permitimos descarregar se não houver objetos ativos (a ser implementado)
    S_OK
}

#[no_mangle]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    if let Err(e) = register::register_server() {
        return e.code();
    }
    if let Err(e) = register::register_profiles() {
        return e.code();
    }
    if let Err(e) = register::register_categories() {
        return e.code();
    }
    S_OK
}

#[no_mangle]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    if let Err(e) = register::unregister_categories() {
        return e.code();
    }
    if let Err(e) = register::unregister_profiles() {
        return e.code();
    }
    if let Err(e) = register::unregister_server() {
        return e.code();
    }
    S_OK
}
