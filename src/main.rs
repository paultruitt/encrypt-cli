mod encrypt_lib;

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
        recipients: Vec<String>
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
        }
        SubCommand::EncryptMessage { message, recipients } => {
            println!("{}", message);
            println!("{:?}", recipients);
            vec!["".to_string()]
        }
    };
    for message in messages {
        println!("{}", message)
    }
}
