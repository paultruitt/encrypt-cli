use age::Recipient;

use crate::encrypt_lib::{encryption, file_management, identity, errors::EncryptCLIError};

pub fn create_keypair_cmd(name: String) -> Result<String, EncryptCLIError> {
    file_management::create_keys_dir()?;
    identity::create_keypair(Some(name))
}

pub fn add_contact_cmd(name: String, pubkey: String) -> Result<(), EncryptCLIError> {
    file_management::create_contacts_dir()?;
    identity::add_contact(&name, &pubkey)
}

pub fn encrypt_message_cmd(message: String, recipients: Vec<String>, pubkeys_passed: bool) -> Result<Vec<u8>, EncryptCLIError> {
    let recipient_objects: Vec<Box<dyn Recipient + Send>> = if pubkeys_passed {
        recipients.iter().filter_map(|recipient| identity::get_recipient_from_str(recipient).ok()).collect()
    } else {
        recipients.iter().filter_map(|recipient| identity::name_to_recipient(recipient).ok()).collect()
    };
    encryption::encrypt_text(&message, recipient_objects)
}

pub fn decrypt_message_cmd(key_name: String, encrypted_message: Vec<u8>) -> Result<String, EncryptCLIError> {
    let id = identity::load_identity(Some(key_name))?;
    encryption::decrypt_buffer(encrypted_message, &id)
}
