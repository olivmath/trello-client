use crate::core::card::{Labels, Steps};
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
}
