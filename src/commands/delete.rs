use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct RemoveCommands {
    #[command(subcommand)]
    pub remove_command: RemoveSubCommands,
}

#[derive(Subcommand)]
pub enum RemoveSubCommands {
    /// Remove a card
    Card {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        id: Option<String>,
    },
}
