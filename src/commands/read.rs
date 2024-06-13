use crate::core::card::{Labels, Steps};
use clap::{ArgGroup, Parser, Subcommand};

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
            .args(&["all", "step", "id", "label"]),
    ))]
    Card {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        step: Option<Steps>,
        #[arg(long)]
        label: Option<Labels>,
        #[arg(long)]
        id: Option<String>,
    },
}
