use std::fmt::Display;

#[derive(Debug)]
pub struct EncryptLibError {
    details: String
}

impl EncryptLibError {
    pub fn new_encryption_error(msg: &str) -> EncryptLibError {
        EncryptLibError {
            details: format!("Encryption Error: {}", msg)
        }
    }

    pub fn new_key_load_error(msg: &str) -> EncryptLibError {
        EncryptLibError {
            details: format!("Key Load Error: {}", msg)
        }
    }

    pub fn new_file_error(msg: &str) -> EncryptLibError {
        EncryptLibError {
            details: format!("File Error: {}", msg)
        }
    }
}

impl Display for EncryptLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
