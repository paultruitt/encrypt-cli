use std::path::PathBuf;
use age::{Recipient, x25519::Identity};

use crate::{encrypt_lib::{encryption, file_management, identity}, errors::EncryptCLIError, logger::Logger};

pub fn create_keypair_cmd(name: String, logger: &Logger) -> Result<String, EncryptCLIError> {
    logger.log("Executing Create Keypair Command");
    file_management::create_keys_dir()?;
    identity::create_keypair(Some(name), logger).map_err(From::from)
}

pub fn add_contact_cmd(name: String, pubkey: String, logger: &Logger) -> Result<(), EncryptCLIError> {
    logger.log("Executing Add Contact Command");
    file_management::create_contacts_dir()?;
    identity::add_contact(&name, &pubkey, logger).map_err(From::from)
}

pub fn encrypt_message_cmd(message: Option<String>, path: Option<PathBuf>, recipients: Vec<String>, pubkeys_passed: bool, outfile: Option<PathBuf>, logger: &Logger) -> Result<Vec<u8>, EncryptCLIError> {
    logger.log("In Encrypt Command");
    logger.debug(&format!("Trying to encrypt for {} Recipients", recipients.len()));
    logger.log("Filtering invalid recipients");
    let recipient_objects: Vec<Box<dyn Recipient + Send>> = if pubkeys_passed {
        recipients.iter().filter_map(|recipient| identity::get_recipient_from_str(recipient, logger).ok()).collect()
    } else {
        recipients.iter().filter_map(|recipient| identity::name_to_recipient(recipient, logger).ok()).collect()
    };
    logger.debug(&format!("Encrypting for {} Recipients", recipient_objects.len()));
    let vec = handle_encryption(message, path, recipient_objects, logger)?;
    if outfile.is_some() {
        logger.log("Writing to outfile");
        file_management::write_to_file(&vec, &outfile.unwrap())?;
    }
    Ok(vec)
}

fn handle_encryption(message: Option<String>, path: Option<PathBuf>, recipients: Vec<Box<dyn Recipient + Send>>, logger: &Logger) -> Result<Vec<u8>, EncryptCLIError> {
    logger.log("Encrypting...");
    if let Some(message) = message {
        logger.debug("Encrypting message");
        encryption::encrypt_bytes(message.as_bytes(), recipients, logger).map_err(From::from)
    } else if path.is_some() {
        let file_bytes = file_management::read_bytes_from_file(&path.unwrap())?;
        logger.debug("Encrypting file's bytes");
        encryption::encrypt_bytes(&file_bytes, recipients, logger).map_err(From::from)
    } else {
        logger.debug("No message to encrypt");
        Err(EncryptCLIError::new_usage_error("Need to include input message or path"))
    }
}

pub fn decrypt_message_cmd(encrypted_message: Option<Vec<u8>>, path: Option<PathBuf>, key_name: String, outfile: Option<PathBuf>, logger: &Logger) -> Result<String, EncryptCLIError> {
    logger.log("In Decrypt Command");
    let id = identity::load_identity(Some(key_name), logger)?;
    let message = handle_decryption(encrypted_message, path, id, logger)?;
    logger.debug("Handling output");
    if outfile.is_some() {
        logger.log("Writing to outfile");
        file_management::write_to_file(&message, &outfile.unwrap())?;
        Ok("Output Written to file".to_string())
    } else {
        match String::from_utf8(message) {
            Ok(m) => Ok(m),
            Err(_e) => Err(EncryptCLIError::new_decoding_error("Failed to convert output to string"))
        }
    }
}

fn handle_decryption(encrypted_message: Option<Vec<u8>>, path: Option<PathBuf>, id: Identity, logger: &Logger) -> Result<Vec<u8>, EncryptCLIError> {
    logger.log("Decrypting...");
    if let Some(encrypted_message) = encrypted_message {
        logger.debug("Decrypting entered bytes");
        encryption::decrypt_bytes(encrypted_message, &id, logger).map_err(From::from)
    } else if path.is_some() {
        logger.debug("Decrypting file");
        let bytes = file_management::read_bytes_from_file(&path.unwrap())?;
        encryption::decrypt_bytes(bytes, &id, logger).map_err(From::from)
    } else {
        logger.debug("No message to decrypt");
        Err(EncryptCLIError::new_usage_error("Need to include input message or path"))
    }
}
