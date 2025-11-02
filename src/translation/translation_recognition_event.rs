use super::TranslationRecognitionResult;
use crate::error::{Result, convert_err};
use crate::ffi::{SPXEVENTHANDLE, SPXRESULTHANDLE, recognizer_recognition_event_get_result};
use crate::speech::RecognitionEvent;
use log::*;
use std::mem::MaybeUninit;

/// Recognition event extending *RecognitionEvent* passed into callbacks *set_recognizing_cb* and *set_recognized_cb*.
#[derive(Debug)]
pub struct TranslationRecognitionEvent {
    pub base: RecognitionEvent,
    pub result: TranslationRecognitionResult,
}

impl TranslationRecognitionEvent {
    /// # Safety
    /// `handle` must be a valid handle to a live translation recognition event.
    pub unsafe fn from_handle(handle: SPXEVENTHANDLE) -> Result<TranslationRecognitionEvent> {
        unsafe {
            let base = RecognitionEvent::from_handle(handle)?;
            let mut result_handle: MaybeUninit<SPXRESULTHANDLE> = MaybeUninit::uninit();
            trace!("calling recognizer_recognition_event_get_result");
            let ret = recognizer_recognition_event_get_result(handle, result_handle.as_mut_ptr());
            convert_err(ret, "TranslationRecognitionEvent::from_handle error")?;
            trace!("called recognizer_recognition_event_get_result");
            let result = TranslationRecognitionResult::from_handle(result_handle.assume_init())?;
            Ok(TranslationRecognitionEvent { base, result })
        }
    }
}
