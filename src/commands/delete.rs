use clap::{Parser, Subcommand};

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
        all: bool,
        #[arg(long)]
        id: Option<String>,
    },
    /// Remove a webhook or all webhooks
    Webhook {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        id: Option<String>,
    },
}
