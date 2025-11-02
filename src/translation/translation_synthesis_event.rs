use super::TranslationSynthesisResult;
use crate::error::{Result, convert_err};
use crate::ffi::{
    SPXEVENTHANDLE, SPXRESULTHANDLE, SmartHandle, recognizer_event_handle_release,
    recognizer_recognition_event_get_result,
};
use std::mem::MaybeUninit;

/// Event passed into translation synthesis callbacks.
#[derive(Debug)]
pub struct TranslationSynthesisEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub result: TranslationSynthesisResult,
}

impl TranslationSynthesisEvent {
    /// # Safety
    /// `handle` must be a valid handle to a live translation synthesis event.
    pub unsafe fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut result_handle: MaybeUninit<SPXRESULTHANDLE> = MaybeUninit::uninit();
            let ret = recognizer_recognition_event_get_result(handle, result_handle.as_mut_ptr());
            convert_err(ret, "TranslationSynthesisEvent::from_handle error")?;
            let result = TranslationSynthesisResult::from_handle(result_handle.assume_init())?;
            Ok(TranslationSynthesisEvent {
                handle: SmartHandle::create(
                    "TranslationSynthesisEvent",
                    handle,
                    recognizer_event_handle_release,
                ),
                result,
            })
        }
    }
}
