use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct GetCommands {
    #[command(subcommand)]
    pub get_command: GetSubCommands,
}

#[derive(Subcommand)]
pub enum GetSubCommands {
    /// Get cards from the board
    Card {
        #[arg(value_name = "my_board", short)]
        my_board: String,
        #[arg(long)]
        all: bool,
        #[arg(long)]
        step: Option<String>,
        #[arg(value_name = "CARD_NAME", short)]
        card_name: Option<String>,
        #[arg(long)]
        id: Option<String>,
    },
}
