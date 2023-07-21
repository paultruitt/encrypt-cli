pub mod arguments;
pub mod commands;
pub mod encrypt_lib;
pub mod errors;
pub mod logger;

use clap::Parser;
use std::vec;

use arguments::{Arguments, SubCommand};

fn main() {
    let args = Arguments::parse();
    let logger = logger::Logger::new(args.verbosity);

    let messages = match args.cmd {
        SubCommand::CreateKeypair { name } => {
            let message = match commands::create_keypair_cmd(name, &logger) {
                Ok(k) => format!("Resulting Pubkey: {}", k),
                Err(e) => format!("Failed to create Keypair: {}", e),
            };
            vec![message]
        }
        SubCommand::AddContact { name, pubkey } => {
            let message = match commands::add_contact_cmd(name, pubkey, &logger) {
                Ok(()) => "Successfully added contact".to_string(),
                Err(e) => format!("Failed to create contact: {}", e),
            };
            vec![message]
        }
        SubCommand::EncryptMessage {
            encrypt_input,
            recipients,
            pubkeys,
            outfile,
        } => {
            let pubkey_result = commands::encrypt_message_cmd(
                encrypt_input.message,
                encrypt_input.file,
                recipients,
                pubkeys,
                outfile,
                &logger,
            );
            if pubkey_result.is_err() {
                vec![format!(
                    "Failed to encrypt: {}",
                    pubkey_result.unwrap_err().to_string()
                )]
            } else {
                let mut msg_string = format!("{:?}", pubkey_result.unwrap());
                msg_string.retain(|c| !(c.is_whitespace() || c == '[' || c == ']'));
                vec![
                    "Encryption Successful".to_string(),
                    format!("Ouput: {:#?}", msg_string),
                ]
            }
        }
        SubCommand::DecryptMessage {
            decrypt_input,
            key,
            outfile,
        } => {
            match commands::decrypt_message_cmd(
                decrypt_input.message,
                decrypt_input.file,
                key,
                outfile,
                &logger,
            ) {
                Ok(s) => vec![
                    "Decryption Successful".to_string(),
                    format!("Ouput: {:#?}", s),
                ],
                Err(e) => vec![format!("Failed to decrypt: {}", e)],
            }
        }
    };
    for message in messages {
        println!("{}", message)
    }
}
