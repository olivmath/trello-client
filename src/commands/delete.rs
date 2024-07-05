use clap::{Parser, Subcommand};

use crate::core::labels::Labels;

#[derive(Parser)]
pub struct RemoveCommands {
    #[command(subcommand)]
    pub remove_command: RemoveSubCommands,
}

#[derive(Subcommand)]
pub enum RemoveSubCommands {
    /// Remove a card or all cards
    Card {
        #[arg(long)]
        id: String,
    },
    /// Remove a webhook or all webhooks
    Webhook {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        id: Option<String>,
    },
    Course {
        #[arg(long)]
        name: Labels,
    },
}
