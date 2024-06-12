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
        #[arg(value_name = "my_board")]
        my_board: String,
        #[arg(value_name = "CARD_NAME")]
        card_name: Option<String>,
        #[arg(long, value_name = "CARD_ID")]
        id: Option<String>,
    },
}
