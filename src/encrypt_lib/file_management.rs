use std::{env, fs, str, path::PathBuf};

use crate::encrypt_lib::errors::EncryptCLIError;

pub fn write_str_to_file(msg: &str, file_path: &PathBuf) -> Result<(), EncryptCLIError> {
    let bytes = Vec::from(msg.as_bytes());
    write_to_file(&bytes, file_path)
}

pub fn read_string_from_file(file_path: &PathBuf) -> Result<String, EncryptCLIError> {
    match fs::read_to_string(file_path) {
        Ok(s) => Ok(s),
        Err(_e) => Err(EncryptCLIError::new_file_error("Failed to read file"))
    }
}

pub fn write_to_file(bytes: &Vec<u8>, file_path: &PathBuf) -> Result<(), EncryptCLIError> {
    match fs::write(file_path, bytes) {
        Ok(()) => Ok(()),
        Err(_e) => Err(EncryptCLIError::new_file_error("Failed to write to file"))
    }
}

pub fn create_keys_dir() -> Result<(), EncryptCLIError> {
    let contacts_dir: PathBuf = get_keys_dir();
    create_dir(&contacts_dir)
}

pub fn create_contacts_dir() -> Result<(), EncryptCLIError> {
    let contacts_dir: PathBuf = get_contnacts_dir();
    create_dir(&contacts_dir)
}

fn create_dir(path: &PathBuf) -> Result<(), EncryptCLIError> {
    match fs::create_dir_all(path) {
        Ok(()) => Ok(()),
        Err(_e) => Err(EncryptCLIError::new_file_error("Failed to create dir"))
    }
}

fn get_config_dir() -> PathBuf {
    let home_dir = env::var_os("HOME").unwrap();
    let mut config_dir = PathBuf::new();
    config_dir.push(home_dir);
    config_dir.push(".config");
    config_dir.push("encrypt-cli");
    return config_dir
}

pub fn get_keys_dir() -> PathBuf {
    let mut config_dir = get_config_dir();
    config_dir.push("keys");
    config_dir
}

pub fn get_contnacts_dir() -> PathBuf {
    let mut config_dir = get_config_dir();
    config_dir.push("contacts");
    config_dir
}
