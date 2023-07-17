mod encrypt_lib;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use encrypt_lib::commands;

/// Simple command line interface for encrypting messages
#[derive(Parser)]
#[clap(author = "Paul Truitt", version, about)]
struct Arguments {

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,
    #[command(subcommand)]
    cmd: SubCommand
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Create a keypair for your user
    CreateKeypair {
        /// Name to give the keypair
        #[arg(short, long, default_value_t = String::from("key"))]
        name: String
    },
    /// Add a user to your contacts
    AddContact {
        /// Name to give the contact
        #[arg(short, long)]
        name: String,
        /// Contact's Public Key
        #[arg(short, long)]
        pubkey: String
    },
    /// Encrypt a Message for a user
    EncryptMessage {
        /// Message to send
        #[arg(short, long)]
        message: String,
        /// Vector of recipients to send to
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        recipients: Vec<String>,
        /// Flag to specify we are passing pubkeys rather than contact names
        #[arg(short, long, action)]
        pubkeys: bool,
        /// File to write the output to
        #[arg(short, long)]
        outfile: Option<PathBuf>
    },
    /// Decrypt a message meant for you
    DecryptMessage {
        /// Key name to decrypt with
        #[arg(short, long)]
        key: String,
        /// Encrypted message
        #[arg(short, long, num_args = 1.., value_delimiter = ',')]
        message: Vec<u8>
    }
}

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
        SubCommand::EncryptMessage { message, recipients, pubkeys, outfile } => {
            let pubkey_result = commands::encrypt_message_cmd(message, recipients, pubkeys, outfile);
            if pubkey_result.is_err() {
                vec![format!("Failed to encrypt: {}", pubkey_result.unwrap_err().to_string())]
            } else {
                let mut msg_string = format!("{:?}", pubkey_result.unwrap());
                msg_string.retain(|c| !(c.is_whitespace() || c == '[' || c == ']'));
                vec!["Encryption Successful".to_string(), format!("Ouput: {:#?}", msg_string)]
            }
        },
        SubCommand::DecryptMessage { key, message } => {
            match commands::decrypt_message_cmd(key, message) {
                Ok(s) => vec!["Decryption Successful".to_string(), format!("Ouput: {:#?}", s)],
                Err(e) => vec![format!("Failed to decrypt: {}", e.to_string())]
            }
        }
    };
    for message in messages {
        println!("{}", message)
    }
}
