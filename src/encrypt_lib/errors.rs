use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct EncryptCLIError {
    details: String
}

impl EncryptCLIError {
    pub fn new_encryption_error(msg: &str) -> EncryptCLIError {
        EncryptCLIError {
            details: format!("Encryption Error: {}", msg)
        }
    }

    pub fn new_key_load_error(msg: &str) -> EncryptCLIError {
        EncryptCLIError {
            details: format!("Key Load Error: {}", msg)
        }
    }

    pub fn new_file_error(msg: &str) -> EncryptCLIError {
        EncryptCLIError {
            details: format!("File Error: {}", msg)
        }
    }
}

impl Display for EncryptCLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EncryptCLIError {
    fn description(&self) -> &str {
        &self.details
    }
}
