use crate::encrypt_lib::{identity, file_management, errors::EncryptCLIError};

pub fn create_keypair_cmd(name: String) -> Result<String, EncryptCLIError> {
    file_management::create_keys_dir()?;
    identity::create_keypair(Some(name))
}

pub fn add_contact_cmd(name: String, pubkey: String) -> Result<(), EncryptCLIError> {
    file_management::create_contacts_dir()?;
    identity::add_contact(&name, &pubkey)
}
