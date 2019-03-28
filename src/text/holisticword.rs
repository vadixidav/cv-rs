//! Holistic word
use super::private::*;
use super::*;
use errors::*;
use std::path::Path;
use *;

/// `OcrHolisticWord` class provides an interface with the tesseract-ocr API
#[derive(Debug)]
pub struct OcrHolisticWord {
    value: *mut native::cvsys_OCRHolisticWordRecognizer,
}

impl OcrHolisticWord {
    /// Creates an instance of the `OcrHolisticWord` class.
    pub fn new<PArch: AsRef<Path>, PWeights: AsRef<Path>, PWords: AsRef<Path>>(
        archive_file: PArch,
        weights_file: PWeights,
        words_file: PWords,
    ) -> Result<Self, Error> {
        let archive_file = path_to_cstring(archive_file)?;
        let weights_file = path_to_cstring(weights_file)?;
        let words_file = path_to_cstring(words_file)?;

        let c_archive_file = archive_file.as_ptr();
        let c_weights_file = weights_file.as_ptr();
        let c_words_file = words_file.as_ptr();

        let result = unsafe { native::cvsys_holistic_new(c_archive_file, c_weights_file, c_words_file) };
        let result: Result<_, String> = result.into();
        let result = result.map_err(CvError::UnknownError)?;
        Ok(Self { value: result })
    }
}

impl Drop for OcrHolisticWord {
    fn drop(&mut self) {
        unsafe {
            native::cvsys_holistic_drop(self.value);
        }
    }
}

impl OcrImpl for OcrHolisticWord {
    fn get_value(&self) -> *mut native::cvsys_BaseOCR {
        self.value as *mut _
    }
}

impl OcrImplInterface for OcrHolisticWord {}
