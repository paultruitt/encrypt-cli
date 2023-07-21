use std::{io::{Write, Read}, iter, vec};

use age::{Encryptor, Decryptor, Recipient, x25519::Identity};

use crate::{encrypt_lib::errors::EncryptLibError, logger::Logger};

pub fn encrypt_bytes(message_bytes: &[u8], recipients: Vec<Box<dyn Recipient + Send>>, logger: &Logger) -> Result<Vec<u8>, EncryptLibError> {
    logger.debug("In encrypt_bytes");
    let encryptor = match Encryptor::with_recipients(recipients) {
        None => {
            logger.debug("Failed to create Encryptor");
            Err(EncryptLibError::new_encryption_error("No recipients passed"))
        },
        Some(enc) => Ok(enc)
    }?;
    let mut encrypted = vec![];
    let mut writer =  match encryptor.wrap_output(&mut encrypted) {
        Err(_e) => {
            logger.debug("Failed to wrap output");
            Err(EncryptLibError::new_encryption_error("Failed to create writer"))
        },
        Ok(w) => Ok(w)
    }?;
    match writer.write_all(message_bytes) {
        Err(_e) => {
            logger.debug("Failed to write all");
            Err(EncryptLibError::new_encryption_error("Failed to write to writer"))
        },
        Ok(w) => Ok(w)
    }?;
    match writer.finish() {
        Err(_e) => {
            logger.debug("Failed to finish writer");
            Err(EncryptLibError::new_encryption_error("Failed to finish writer"))
        },
        Ok(w) => Ok(w)
    }?;
    Ok(encrypted)
}

pub fn decrypt_bytes(encrypted: Vec<u8>, identity: &Identity, logger: &Logger) -> Result<Vec<u8>, EncryptLibError> {
    logger.debug("In decrypt_bytes");
    let decryptor = match Decryptor::new(&encrypted[..]) {
        Ok(Decryptor::Recipients(d)) => Ok(d),
        Ok(_) => unreachable!(),
        Err(_e) => {
            logger.debug("Failed to create decryptor");
            Err(EncryptLibError::new_encryption_error("Failed to create decrypter"))
        }
    }?;
    let mut decrypted = vec![];
    let mut reader = match decryptor.decrypt(iter::once(identity as &dyn age::Identity)) {
        Ok(r) => Ok(r),
        Err(_e) => {
            logger.debug("Failed to decrypt message");
            Err(EncryptLibError::new_encryption_error("Failed to create reader"))
        }
    }?;
    let _ = reader.read_to_end(&mut decrypted);
    Ok(decrypted)
}
