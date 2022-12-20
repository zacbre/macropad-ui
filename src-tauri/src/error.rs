use anyhow::Error;
use nvml_wrapper::error::NvmlError;

#[derive(Debug)]
pub struct HidError {
    pub message: String
}

impl HidError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

impl From<hidapi::HidError> for HidError {
    fn from(value: hidapi::HidError) -> Self {
        HidError::new(value.to_string())
    }
}

impl From<anyhow::Error> for HidError {
    fn from(value: Error) -> Self {
        HidError::new(value.to_string())
    }
}

impl From<serde_json::Error> for HidError {
    fn from(value: serde_json::Error) -> Self {
        HidError::new(value.to_string())
    }
}

impl From<std::io::Error> for HidError {
    fn from(value: std::io::Error) -> Self {
        HidError::new(value.to_string())
    }
}

impl From<NvmlError> for HidError {
    fn from(value: NvmlError) -> Self {
        HidError::new(value.to_string())
    }
}