use crate::core::card::Steps;
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
        #[arg(long)]
        label: String,
        #[arg(long, value_enum)]
        step: Steps,
    },
}
