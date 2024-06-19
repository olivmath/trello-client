use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct MoveCommands {
    #[command(subcommand)]
    pub move_command: MoveSubCommands,
}

#[derive(Subcommand)]
pub enum MoveSubCommands {
    /// Move a card
    Card {
        #[arg(value_name = "my_board")]
        my_board: String,
        #[arg(long)]
        next: bool,
        #[arg(long)]
        back: bool,
        #[arg(long, value_name = "CARD_ID")]
        id: String,
    },
}

#[derive(Parser)]
pub struct EditCommands {
    #[command(subcommand)]
    pub edit_command: EditSubCommands,
}

#[derive(Subcommand)]
pub enum EditSubCommands {
    /// Edit a card
    Card {
        #[arg(value_name = "my_board")]
        my_board: String,
        #[arg(long, value_name = "CARD_ID")]
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        label: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
    /// Edit a webhook
    Webhook {
        #[arg(long)]
        id: String,
        #[arg(long)]
        callback: String,
        #[arg(long)]
        active: Option<bool>,
    },
}
