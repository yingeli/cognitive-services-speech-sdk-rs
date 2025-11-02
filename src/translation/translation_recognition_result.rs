use crate::common::{PropertyCollection, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    recognizer_result_handle_release, result_get_duration, result_get_offset,
    result_get_property_bag, result_get_reason, result_get_result_id, result_get_text,
    translation_text_result_get_translation, translation_text_result_get_translation_count,
    SmartHandle, SPXPROPERTYBAGHANDLE, SPXRESULTHANDLE,
};
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;

/// Represents translation recognition result contained within callback event *TranslationRecognitionEvent*.
pub struct TranslationRecognitionResult {
    pub handle: SmartHandle<SPXRESULTHANDLE>,
    pub result_id: String,
    pub reason: ResultReason,
    pub text: String,
    pub duration: String, //TBD: change to duration
    pub offset: String,   // TBD: change to duration
    pub translations: HashMap<String, String>,
    pub properties: PropertyCollection,
}

impl fmt::Debug for TranslationRecognitionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TranslationRecognitionResult")
            .field("result_id", &self.result_id)
            .field("reason", &self.reason)
            .field("text", &self.text)
            .field("duration", &self.duration)
            .field("offset", &self.offset)
            .field("translations", &self.translations)
            .finish()
    }
}

impl TranslationRecognitionResult {
    /// # Safety
    /// `handle` must be a valid handle to a live translation recognition result.
    pub unsafe fn from_handle(handle: SPXRESULTHANDLE) -> Result<TranslationRecognitionResult> {
        unsafe {
            let mut c_buf = [0; 2048];
            let mut ret = result_get_result_id(handle, c_buf.as_mut_ptr(), c_buf.len() as u32 - 1);
            convert_err(
                ret,
                "TranslationRecognitionResult::from_handle(result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf.as_ptr()).to_str()?.to_owned();

            let mut reason = 0;
            ret = result_get_reason(handle, &mut reason);
            convert_err(
                ret,
                "TranslationRecognitionResult::from_handle(result_get_reason) error",
            )?;

            // let mut c_buf2 = [0; 2048];
            ret = result_get_text(handle, c_buf.as_mut_ptr(), c_buf.len() as u32 - 1);
            convert_err(
                ret,
                "TranslationRecognitionResult::from_handle(result_get_text) error",
            )?;
            let result_text = CStr::from_ptr(c_buf.as_ptr()).to_str()?.to_owned();

            let mut duration: u64 = 0;
            ret = result_get_duration(handle, &mut duration);
            convert_err(
                ret,
                "TranslationRecognitionResult::from_handle(result_get_duration) error",
            )?;

            let mut offset: u64 = 0;
            ret = result_get_offset(handle, &mut offset);
            convert_err(
                ret,
                "TranslationRecognitionResult::from_handle(result_get_offset) error",
            )?;

            let mut translation_count: usize = 0;
            ret = translation_text_result_get_translation_count(handle, &mut translation_count);
            convert_err(
                ret,
                "TranslationRecognitionResult::from_handle(translation_text_result_get_translation_count) error",
            )?;

            let mut max_lang_size = 0;
            let mut max_text_size = 0;
            for i in 0..translation_count {
                let mut lang_size = 0;
                let mut text_size = 0;
                ret = translation_text_result_get_translation(
                    handle,
                    i,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut lang_size,
                    &mut text_size,
                );
                convert_err(
                    ret,
                    "TranslationRecognitionResult::from_handle(translation_text_result_get_translation - size) error",
                )?;
                max_lang_size = max_lang_size.max(lang_size);
                max_text_size = max_text_size.max(text_size);
            }

            let mut translations = HashMap::new();
            let mut lang_buf = vec![0; max_lang_size];
            let mut text_buf = vec![0; max_text_size];
            for i in 0..translation_count {
                ret = translation_text_result_get_translation(
                    handle,
                    i,
                    lang_buf.as_mut_ptr(),
                    text_buf.as_mut_ptr(),
                    &mut max_lang_size,
                    &mut max_text_size,
                );
                convert_err(
                    ret,
                    "TranslationRecognitionResult::from_handle(translation_text_result_get_translation) error",
                )?;
                let lang = CStr::from_ptr(lang_buf.as_ptr()).to_str()?.to_owned();
                let text = CStr::from_ptr(text_buf.as_ptr()).to_str()?.to_owned();
                translations.insert(lang, text);
            }

            let mut properties_handle: MaybeUninit<SPXPROPERTYBAGHANDLE> = MaybeUninit::uninit();
            ret = result_get_property_bag(handle, properties_handle.as_mut_ptr());
            convert_err(
                ret,
                "TranslationRecognitionResult::from_handle(result_get_property_bag) error",
            )?;
            let properties = PropertyCollection::from_handle(properties_handle.assume_init());

            Ok(TranslationRecognitionResult {
                handle: SmartHandle::create(
                    "TranslationRecognitionResult",
                    handle,
                    recognizer_result_handle_release,
                ),
                result_id,
                reason: reason.into(),
                text: result_text,
                duration: (duration).to_string(),
                offset: (offset).to_string(),
                translations,
                properties,
            })
        }
    }
}
