pub mod arguments;
pub mod commands;
pub mod encrypt_lib;
pub mod errors;

use std::vec;
use clap::Parser;

use arguments::{Arguments, SubCommand};

fn main() {
    let args = Arguments::parse();

    let messages = match args.cmd {
        SubCommand::CreateKeypair { name } => {
            let message = match commands::create_keypair_cmd(name) {
                Ok(k) => format!("Resulting Pubkey: {}", k),
                Err(e) => format!("Failed to create Keypair: {}", e.to_string())
            };
            vec![message]
        },
        SubCommand::AddContact { name, pubkey } => {
            let message = match commands::add_contact_cmd(name, pubkey) {
                Ok(()) => format!("Successfully added contact"),
                Err(e) => format!("Failed to create contact: {}", e.to_string())
            };
            vec![message]
        },
        SubCommand::EncryptMessage { encrypt_input, recipients, pubkeys, outfile } => {
            let pubkey_result = commands::encrypt_message_cmd(encrypt_input.message, encrypt_input.file, recipients, pubkeys, outfile);
            if pubkey_result.is_err() {
                vec![format!("Failed to encrypt: {}", pubkey_result.unwrap_err().to_string())]
            } else {
                let mut msg_string = format!("{:?}", pubkey_result.unwrap());
                msg_string.retain(|c| !(c.is_whitespace() || c == '[' || c == ']'));
                vec!["Encryption Successful".to_string(), format!("Ouput: {:#?}", msg_string)]
            }
        },
        SubCommand::DecryptMessage { decrypt_input, key, outfile } => {
            match commands::decrypt_message_cmd(decrypt_input.message, decrypt_input.file, key, outfile) {
                Ok(s) => vec!["Decryption Successful".to_string(), format!("Ouput: {:#?}", s)],
                Err(e) => vec![format!("Failed to decrypt: {}", e.to_string())]
            }
        }
    };
    for message in messages {
        println!("{}", message)
    }
}
