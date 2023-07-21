use std::fmt::Display;

use crate::encrypt_lib::errors::EncryptLibError;

#[derive(Debug)]
pub struct EncryptCLIError {
    details: String,
}

impl EncryptCLIError {
    pub fn new_usage_error(msg: &str) -> EncryptCLIError {
        EncryptCLIError {
            details: format!("Usage Error: {}", msg),
        }
    }

    pub fn new_decoding_error(msg: &str) -> EncryptCLIError {
        EncryptCLIError {
            details: format!("Decoding Error: {}", msg),
        }
    }
}

impl Display for EncryptCLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<EncryptLibError> for EncryptCLIError {
    fn from(value: EncryptLibError) -> Self {
        EncryptCLIError {
            details: value.to_string(),
        }
    }
}
