use clap::{ArgGroup, Parser, Subcommand};
use crate::core::card::Steps;

#[derive(Parser)]
pub struct GetCommands {
    #[command(subcommand)]
    pub get_command: GetSubCommands,
}

#[derive(Subcommand)]
pub enum GetSubCommands {
    /// Get cards from the board
    #[clap(group(
        ArgGroup::new("card_options")
            .required(true)
            .args(&["all", "step", "id"]),
    ))]
    Card {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        step: Option<Steps>,
        #[arg(long)]
        id: Option<String>,
    },
}
