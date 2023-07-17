use std::{io::{Write, Read}, iter, str, vec};

use age::{Encryptor, Decryptor, Recipient, x25519::Identity};

use crate::encrypt_lib::errors::EncryptCLIError;

pub fn encrypt_text(msg: &str, recipients: Vec<Box<dyn Recipient + Send>>) -> Result<Vec<u8>, EncryptCLIError> {
    let encryptor = match Encryptor::with_recipients(recipients) {
        None => Err(EncryptCLIError::new_encryption_error("No recipients passed")),
        Some(enc) => Ok(enc)
    }?;
    let mut encrypted = vec![];
    let mut writer =  match encryptor.wrap_output(&mut encrypted) {
        Err(_e) => Err(EncryptCLIError::new_encryption_error("Failed to create writer")),
        Ok(w) => Ok(w)
    }?;
    match writer.write_all(msg.as_bytes()) {
        Err(_e) => Err(EncryptCLIError::new_encryption_error("Failed to write to writer")),
        Ok(w) => Ok(w)
    }?;
    match writer.finish() {
        Err(_e) => Err(EncryptCLIError::new_encryption_error("Failed to finish writer")),
        Ok(w) => Ok(w)
    }?;
    Ok(encrypted)
}

pub fn decrypt_buffer(encrypted: Vec<u8>, identity: &Identity) -> Result<String, EncryptCLIError> {
    let decryptor = match Decryptor::new(&encrypted[..]) {
        Ok(Decryptor::Recipients(d)) => Ok(d),
        Ok(_) => unreachable!(),
        Err(_e) => Err(EncryptCLIError::new_encryption_error("Failed to create decrypter"))
    }?;
    let mut decrypted = vec![];
    let mut reader = match decryptor.decrypt(iter::once(identity as &dyn age::Identity)) {
        Ok(r) => Ok(r),
        Err(_e) => Err(EncryptCLIError::new_encryption_error("Failed to create reader"))
    }?;
    let _ = reader.read_to_end(&mut decrypted);
    return match String::from_utf8(decrypted) {
        Ok(m) => Ok(m),
        Err(_e) => Err(EncryptCLIError::new_encryption_error("Couldn't convert output to UTF-8"))
    }
}
