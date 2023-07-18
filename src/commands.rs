use std::path::PathBuf;
use age::{Recipient, x25519::Identity};

use crate::{encrypt_lib::{encryption, file_management, identity}, errors::EncryptCLIError};

pub fn create_keypair_cmd(name: String) -> Result<String, EncryptCLIError> {
    file_management::create_keys_dir()?;
    identity::create_keypair(Some(name)).map_err(From::from)
}

pub fn add_contact_cmd(name: String, pubkey: String) -> Result<(), EncryptCLIError> {
    file_management::create_contacts_dir()?;
    identity::add_contact(&name, &pubkey).map_err(From::from)
}

pub fn encrypt_message_cmd(message: Option<String>, path: Option<PathBuf>, recipients: Vec<String>, pubkeys_passed: bool, outfile: Option<PathBuf>) -> Result<Vec<u8>, EncryptCLIError> {
    let recipient_objects: Vec<Box<dyn Recipient + Send>> = if pubkeys_passed {
        recipients.iter().filter_map(|recipient| identity::get_recipient_from_str(recipient).ok()).collect()
    } else {
        recipients.iter().filter_map(|recipient| identity::name_to_recipient(recipient).ok()).collect()
    };
    let vec = handle_encryption(message, path, recipient_objects)?;
    if outfile.is_some() {
        file_management::write_to_file(&vec, &outfile.unwrap())?;
    }
    Ok(vec)
}

fn handle_encryption(message: Option<String>, path: Option<PathBuf>, recipients: Vec<Box<dyn Recipient + Send>>) -> Result<Vec<u8>, EncryptCLIError> {
    if message.is_some() {
        encryption::encrypt_bytes(&message.unwrap().as_bytes().to_vec(), recipients).map_err(From::from)
    } else if path.is_some() {
        let file_bytes = file_management::read_bytes_from_file(&path.unwrap())?;
        encryption::encrypt_bytes(&file_bytes, recipients).map_err(From::from)
    } else {
        Err(EncryptCLIError::new_usage_error("Need to include input message or path"))
    }
}

pub fn decrypt_message_cmd(encrypted_message: Option<Vec<u8>>, path: Option<PathBuf>, key_name: String, outfile: Option<PathBuf>) -> Result<String, EncryptCLIError> {
    let id = identity::load_identity(Some(key_name))?;
    let message = handle_decryption(encrypted_message, path, id)?;
    if outfile.is_some() {
        file_management::write_to_file(&message, &outfile.unwrap())?;
        Ok("Output Written to file".to_string())
    } else {
        match String::from_utf8(message) {
            Ok(m) => Ok(m),
            Err(_e) => Err(EncryptCLIError::new_decoding_error("Failed to convert output to string"))
        }
    }
}

fn handle_decryption(encrypted_message: Option<Vec<u8>>, path: Option<PathBuf>, id: Identity) -> Result<Vec<u8>, EncryptCLIError> {
    if encrypted_message.is_some() {
        encryption::decrypt_bytes(encrypted_message.unwrap(), &id).map_err(From::from)
    } else if path.is_some() {
        let bytes = file_management::read_bytes_from_file(&path.unwrap())?;
        encryption::decrypt_bytes(bytes, &id).map_err(From::from)
    } else {
        Err(EncryptCLIError::new_usage_error("Need to include input message or path"))
    }
}
