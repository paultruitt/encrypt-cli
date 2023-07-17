use std::str::FromStr;
use age::{x25519::{Identity, Recipient}, secrecy::ExposeSecret};

use crate::encrypt_lib::file_management;
use crate::encrypt_lib::errors::EncryptCLIError;

pub fn add_contact(name: &str, key: &str) -> Result<(), EncryptCLIError> {
    let mut contacts_path = file_management::get_contnacts_dir();
    contacts_path.push(name);
    file_management::write_str_to_file(key, &contacts_path)
}

pub fn load_identity(name: Option<String>) -> Result<Identity, EncryptCLIError> {
    let used_name = name.unwrap_or("key".to_string());
    let mut config_dir = file_management::get_keys_dir();
    config_dir.push(used_name);
    let key_contents = file_management::read_string_from_file(&config_dir)?;
    get_identity_from_str(&key_contents)
}

pub fn create_keypair(pair_name: Option<String>) -> Result<String, EncryptCLIError> {
    let new_keypair = Identity::generate();
    let name = pair_name.unwrap_or("key".to_string());
    save_private(&new_keypair, &name)?;
    save_pubkey(&new_keypair, &name)
}

pub fn name_to_recipient(recipient_name: &str) -> Result<Box<dyn age::Recipient + Send>, EncryptCLIError> {
    let mut contacts_dir = file_management::get_contnacts_dir();
    contacts_dir.push(recipient_name);
    let pubkey = file_management::read_string_from_file(&contacts_dir)?;
    get_recipient_from_str(&pubkey)
}

pub fn get_recipient_from_str(pubkey_str: &str) -> Result<Box<dyn age::Recipient + Send>, EncryptCLIError> {
    let recipient = match Recipient::from_str(pubkey_str) {
        Err(_e) => Err(EncryptCLIError::new_key_load_error("Failed to create Recipient from Key")),
        Ok(r) => Ok(r)
    }?;
    Ok(Box::new(recipient))
}

fn get_identity_from_str(privkey_str: &str) -> Result<Identity, EncryptCLIError> {
    match Identity::from_str(privkey_str) {
        Ok(i) => Ok(i),
        Err(e) => Err(EncryptCLIError::new_key_load_error(&format!("Failed to create Identity from private: {}", e)))
    }
}

fn save_pubkey(identity: &Identity, key_name: &String) -> Result<String, EncryptCLIError> {
    let public_key = identity.to_public().to_string();
    let mut config_path = file_management::get_keys_dir();
    config_path.push(format!("{}.pub", key_name));
    let save_result = file_management::write_str_to_file(&public_key, &config_path);
    return match save_result {
        Err(e) => Err(e),
        Ok(()) => Ok(public_key)
    }
}

fn save_private(identity: &Identity, key_name: &String) -> Result<(), EncryptCLIError> {
    let secret = identity.to_string();
    let private_key = secret.expose_secret();
    let mut config_path = file_management::get_keys_dir();
    config_path.push(key_name);
    file_management::write_str_to_file(private_key, &config_path)
}
