use std::str::FromStr;
use age::{x25519::{Identity, Recipient}, secrecy::ExposeSecret};

use crate::{encrypt_lib::{errors::EncryptLibError, file_management}, logger::Logger};

pub fn add_contact(name: &str, key: &str, logger: &Logger) -> Result<(), EncryptLibError> {
    logger.debug("Adding contact File");
    let mut contacts_path = file_management::get_contnacts_dir();
    contacts_path.push(name);
    file_management::write_str_to_file(key, &contacts_path)
}

pub fn load_identity(name: Option<String>, logger: &Logger) -> Result<Identity, EncryptLibError> {
    logger.debug("Loading Identity File");
    let used_name = name.unwrap_or("key".to_string());
    let mut config_dir = file_management::get_keys_dir();
    config_dir.push(used_name);
    let key_contents = file_management::read_string_from_file(&config_dir)?;
    get_identity_from_str(&key_contents, logger)
}

pub fn create_keypair(pair_name: Option<String>, logger: &Logger) -> Result<String, EncryptLibError> {
    logger.debug("Generating Keypair");
    let new_keypair = Identity::generate();
    let name = pair_name.unwrap_or("key".to_string());
    save_private(&new_keypair, &name, logger)?;
    save_pubkey(&new_keypair, &name, logger)
}

pub fn name_to_recipient(recipient_name: &str, logger: &Logger) -> Result<Box<dyn age::Recipient + Send>, EncryptLibError> {
    logger.debug("Creating Recipient object from name");
    let mut contacts_dir = file_management::get_contnacts_dir();
    contacts_dir.push(recipient_name);
    let pubkey = file_management::read_string_from_file(&contacts_dir)?;
    get_recipient_from_str(&pubkey, logger)
}

pub fn get_recipient_from_str(pubkey_str: &str, logger: &Logger) -> Result<Box<dyn age::Recipient + Send>, EncryptLibError> {
    logger.debug("Creating recipient from pubkey string");
    let recipient = match Recipient::from_str(pubkey_str) {
        Err(_e) => Err(EncryptLibError::new_key_load_error("Failed to create Recipient from Key")),
        Ok(r) => Ok(r)
    }?;
    Ok(Box::new(recipient))
}

fn get_identity_from_str(privkey_str: &str, logger: &Logger) -> Result<Identity, EncryptLibError> {
    logger.debug("Creating Identity from private key string");
    match Identity::from_str(privkey_str) {
        Ok(i) => Ok(i),
        Err(e) => Err(EncryptLibError::new_key_load_error(&format!("Failed to create Identity from private: {}", e)))
    }
}

fn save_pubkey(identity: &Identity, key_name: &String, logger: &Logger) -> Result<String, EncryptLibError> {
    logger.debug("Saving public key to file");
    let public_key = identity.to_public().to_string();
    let mut config_path = file_management::get_keys_dir();
    config_path.push(format!("{}.pub", key_name));
    let save_result = file_management::write_str_to_file(&public_key, &config_path);
    match save_result {
        Err(e) => Err(e),
        Ok(()) => Ok(public_key)
    }
}

fn save_private(identity: &Identity, key_name: &String, logger: &Logger) -> Result<(), EncryptLibError> {
    logger.debug("Saving private key to file");
    let secret = identity.to_string();
    let private_key = secret.expose_secret();
    let mut config_path = file_management::get_keys_dir();
    config_path.push(key_name);
    file_management::write_str_to_file(private_key, &config_path)
}
