use super::TranslationRecognitionEvent;
use crate::common::{CancellationErrorCode, CancellationReason, PropertyId};
use crate::error::{Result, convert_err};
use crate::ffi::{SPXEVENTHANDLE, result_get_canceled_error_code, result_get_reason_canceled};
use log::*;

/// Recognition event extending *TranslationRecognitionEvent* passed into callback *set_canceled_cb*.
#[derive(Debug)]
pub struct TranslationRecognitionCanceledEvent {
    pub base: TranslationRecognitionEvent,
    pub reason: CancellationReason,
    pub error_code: CancellationErrorCode,
    pub error_details: String,
}

impl TranslationRecognitionCanceledEvent {
    /// # Safety
    /// `handle` must be a valid handle to a live translation recognition cancelled event.
    pub unsafe fn from_handle(
        handle: SPXEVENTHANDLE,
    ) -> Result<TranslationRecognitionCanceledEvent> {
        unsafe {
            let base = TranslationRecognitionEvent::from_handle(handle)?;
            let mut reason = 0;
            let ret = result_get_reason_canceled(base.result.handle.inner(), &mut reason);
            convert_err(
                ret,
                "TranslationRecognitionCanceledEvent::from_handle(result_get_reason_canceled) error",
            )?;

            let mut error_code = 0;
            let ret = result_get_canceled_error_code(base.result.handle.inner(), &mut error_code);
            convert_err(
                ret,
                "TranslationRecognitionCanceledEvent::from_handle(result_get_canceled_error_code) error",
            )?;

            let error_details;
            let error_details_res = base.result.properties.get_property(
                PropertyId::SpeechServiceResponseJsonErrorDetails,
                "".to_string(),
            );
            if let Err(err) = error_details_res {
                warn!(
                    "Error when getting SpeechServiceResponseJsonErrorDetails {:?}",
                    err
                );
                error_details = "".to_owned();
            } else {
                error_details = error_details_res.unwrap();
            }

            Ok(TranslationRecognitionCanceledEvent {
                base,
                reason: reason.into(),
                error_code: error_code.into(),
                error_details,
            })
        }
    }
}
