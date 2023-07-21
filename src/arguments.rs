use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Simple command line interface for encrypting messages
#[derive(Parser)]
#[clap(author = "Paul Truitt", version, about)]
pub struct Arguments {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbosity: u8,
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Create a keypair for your user
    CreateKeypair {
        /// Name to give the keypair
        #[arg(short, long, default_value_t = String::from("key"))]
        name: String,
    },
    /// Add a user to your contacts
    AddContact {
        /// Name to give the contact
        #[arg(short, long)]
        name: String,
        /// Contact's Public Key
        #[arg(short, long)]
        pubkey: String,
    },
    /// Encrypt a Message for a user
    EncryptMessage {
        #[command(flatten)]
        encrypt_input: EncryptInput,
        /// Vector of recipients to send to
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        recipients: Vec<String>,
        /// Flag to specify we are passing pubkeys rather than contact names
        #[arg(short, long, action)]
        pubkeys: bool,
        /// File to write the output to
        #[arg(short, long)]
        outfile: Option<PathBuf>,
    },
    /// Decrypt a message meant for you
    DecryptMessage {
        #[command(flatten)]
        decrypt_input: DecryptInput,
        /// Key name to decrypt with
        #[arg(short, long)]
        key: String,
        /// File to write the output to
        #[arg(short, long)]
        outfile: Option<PathBuf>,
    },
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct EncryptInput {
    /// Input text
    #[arg(long, short)]
    pub message: Option<String>,
    /// File containing input
    #[arg(long, short)]
    pub file: Option<PathBuf>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DecryptInput {
    /// Encrypted bytes (comma seperated)
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    pub message: Option<Vec<u8>>,
    /// Encrypted file
    #[arg(long, short)]
    pub file: Option<PathBuf>,
}
