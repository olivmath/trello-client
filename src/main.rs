use clap::Parser;
use dotenv::dotenv;
use trello_client::commands::{
    add_commands, edit_commands, get_commands, move_commands, remove_commands, Cli, Commands,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(add) => add_commands(&add).await,
        Commands::Get(get) => get_commands(&get),
        Commands::Move(mov) => move_commands(&mov),
        Commands::Edit(edit) => edit_commands(&edit),
        Commands::Remove(remove) => remove_commands(&remove),
    }
}
