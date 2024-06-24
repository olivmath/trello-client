use crate::core::{labels::Labels, steps::Steps};
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct AddCommands {
    #[command(subcommand)]
    pub add_command: AddSubCommands,
}

#[derive(Subcommand)]
pub enum AddSubCommands {
    /// Add a new card
    Card {
        #[arg(long)]
        name: String,
        #[arg(long, value_enum)]
        label: Labels,
        #[arg(long, value_enum)]
        step: Steps,
    },
    /// Add cards from a json file
    Json {
        #[arg(long)]
        path: String,
    },
    /// Add a new webhook
    Webhook {
        #[arg(long)]
        callback: String,
        #[arg(long)]
        name: String,
    },
}
