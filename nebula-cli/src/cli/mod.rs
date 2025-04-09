pub mod command;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, author, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Initialize {
        /// Onion address of the relay to connect to
        #[arg(long)]
        relay_url: String,
    },
    /// Create a new identity if none exists
    Auth,
}
