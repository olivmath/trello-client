use crate::core::labels::Labels;
use crate::core::steps::Steps;
use clap::{ArgGroup, Parser, Subcommand};

#[derive(Parser)]
pub struct GetCommands {
    #[command(subcommand)]
    pub get_command: GetSubCommands,
}

#[derive(Subcommand)]
pub enum GetSubCommands {
    /// Get all labels from the board
    Label {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        step: Option<Steps>,
    },
    /// Get cards from the board
    #[clap(group(
        ArgGroup::new("card_options")
        .required(true)
        .args(& ["all", "step", "id", "label"]),
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
    /// Get Webhook by id
    #[clap(group(
    ArgGroup::new("webhook_options")
    .required(true)
    .args(& ["all", "id"]),
))]
    Webhook {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        all: bool,
    },
}
