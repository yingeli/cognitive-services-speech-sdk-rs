use crate::common::{PropertyCollection, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    recognizer_result_handle_release, result_get_property_bag, result_get_reason,
    result_get_result_id, translation_synthesis_result_get_audio_data, SmartHandle,
    SPXPROPERTYBAGHANDLE, SPXRESULTHANDLE,
};
use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;

/// Represents translation synthesis result contained in TranslationSynthesisEvent callback event.
pub struct TranslationSynthesisResult {
    pub handle: SmartHandle<SPXRESULTHANDLE>,
    pub result_id: String,
    pub reason: ResultReason,
    pub audio: Vec<u8>,
    // pub audio_duration_ms: u64,
    pub properties: PropertyCollection,
}

impl fmt::Debug for TranslationSynthesisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let audio_truncated = if self.audio.len() > 10 {
            &self.audio[..10]
        } else {
            &self.audio[..]
        };
        f.debug_struct("TranslationSynthesisResult")
            .field("result_id", &self.result_id)
            .field("reason", &self.reason)
            .field("audio", &format!("(Truncated): {:?}", &audio_truncated))
            // .field("audio", &self.audio)
            .finish()
    }
}

impl TranslationSynthesisResult {
    /// # Safety
    /// `handle` must be a valid handle to a live translation synthesis result.
    pub unsafe fn from_handle(handle: SPXRESULTHANDLE) -> Result<Self> {
        unsafe {
            let mut c_buf = [0; 1024];
            let mut ret = result_get_result_id(handle, c_buf.as_mut_ptr(), c_buf.len() as u32 - 1);
            convert_err(
                ret,
                "TranslationSynthesisResult::from_handle(result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf.as_ptr()).to_str()?.to_owned();

            let mut reason = 0;
            ret = result_get_reason(handle, &mut reason);
            convert_err(
                ret,
                "TranslationSynthesisResult::from_handle(synth_result_get_reason) error",
            )?;

            let mut audio_length: usize = 0;
            translation_synthesis_result_get_audio_data(
                handle,
                std::ptr::null_mut(),
                &mut audio_length,
            );

            let mut c_buf2_vec = vec![0u8; audio_length];
            let c_buf2: *mut u8 = &mut c_buf2_vec[..] as *const _ as *mut u8;
            ret = translation_synthesis_result_get_audio_data(handle, c_buf2, &mut audio_length);
            convert_err(
                ret,
                "TranslationSynthesisResult::from_handle(translation_synthesis_result_get_audio_data) error",
            )?;

            let slice_buffer = std::slice::from_raw_parts_mut(c_buf2, audio_length);

            let mut properties_handle: MaybeUninit<SPXPROPERTYBAGHANDLE> = MaybeUninit::uninit();
            ret = result_get_property_bag(handle, properties_handle.as_mut_ptr());
            convert_err(
                ret,
                "TranslationSynthesisResult::from_handle(result_get_property_bag) error",
            )?;
            let properties = PropertyCollection::from_handle(properties_handle.assume_init());

            let translation_synthesis_result = TranslationSynthesisResult {
                handle: SmartHandle::create(
                    "TranslationSynthesisResult",
                    handle,
                    recognizer_result_handle_release,
                ),
                result_id,
                reason: reason.into(),
                audio: slice_buffer.to_vec(),
                // audio_duration_ms: audio_duration,
                properties,
            };
            Ok(translation_synthesis_result)
        }
    }
}
