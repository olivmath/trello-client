use clap::Parser;
use dotenv::dotenv;
use tc::commands::{
    add_commands, edit_commands, get_commands, move_commands, remove_commands, Cli, Commands,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(add) => add_commands(add).await,
        Commands::Get(get) => get_commands(get).await,
        Commands::Move(mov) => move_commands(mov),
        Commands::Edit(edit) => edit_commands(edit).await,
        Commands::Remove(remove) => remove_commands(remove).await,
    }
}
