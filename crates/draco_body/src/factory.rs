use windows::core::*;
use windows::Win32::System::Com::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::TextServices::*;
use crate::tip::PtBrTip;

#[implement(IClassFactory)]
pub struct PtBrTipFactory;

impl IClassFactory_Impl for PtBrTipFactory {
    fn CreateInstance(
        &self,
        punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        if punkouter.is_some() {
            return Err(Error::from(CLASS_E_NOAGGREGATION));
        }

        let tip: ITfTextInputProcessorEx = PtBrTip::new().into();
        unsafe { tip.query(riid, ppvobject).ok() }
    }

    fn LockServer(&self, flock: BOOL) -> Result<()> {
        let _ = flock;
        Ok(())
    }
}
