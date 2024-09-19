pub mod create;
pub mod delete;
pub mod read;
pub mod update;

use crate::commands::create::AddSubCommands;
use crate::commands::delete::RemoveSubCommands;
use crate::commands::read::GetSubCommands;
use crate::commands::update::{EditSubCommands, MoveSubCommands};
use crate::core::json_reader;
use crate::core::trello_repository::cards::Card;
use crate::core::trello_repository::labels::Labels; // Add this line
use crate::core::trello_repository::webhooks::Webhook;
use clap::{Parser, Subcommand};
use create::AddCommands;
use delete::RemoveCommands;
use read::GetCommands;
use update::{EditCommands, MoveCommands};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add resources
    Add(AddCommands),
    /// Get resources
    Get(GetCommands),
    /// Move cards
    Move(MoveCommands),
    /// Edit resources
    Edit(EditCommands),
    /// Remove resources
    Remove(RemoveCommands),
}

pub async fn add_commands(add: &AddCommands) {
    match &add.add_command {
        AddSubCommands::Card { name, label, step } => {
            Card::add_card(name.to_owned(), label.to_owned(), step.to_owned()).await;
        }
        AddSubCommands::Json { path } => {
            json_reader::process_cads_from_json(path).await;
        }
        AddSubCommands::Webhook { callback, name } => {
            Webhook::add_webhook(name, callback).await;
        }
    }
}

pub async fn get_commands(get: &GetCommands) {
    match &get.get_command {
        GetSubCommands::Card {
            all,
            step,
            id,
            label,
        } => Card::get_card(*all, step, label, id).await,

        GetSubCommands::Webhook { id, all } => Webhook::get_webhook(id.to_owned(), *all).await,
        GetSubCommands::Label { all, step } => Labels::get_labels(*all, step).await,
    }
}

pub async fn edit_commands(edit: &EditCommands) {
    match &edit.edit_command {
        EditSubCommands::Card {
            my_board,
            id,
            name,
            label,
            color,
        } => {
            Card::edit_card(
                my_board,
                id,
                name.as_deref(),
                label.as_deref(),
                color.as_deref(),
            );
        }
        EditSubCommands::Webhook {
            id,
            callback,
            active,
        } => {
            Webhook::update_webhook_by_id(id, callback, *active).await;
        }
    }
}

pub fn move_commands(mov: &MoveCommands) {
    match &mov.move_command {
        MoveSubCommands::Card {
            my_board,
            next,
            back,
            id,
        } => {
            Card::move_card(my_board, *next, *back, id);
        }
    }
}

pub async fn remove_commands(remove: &RemoveCommands) {
    match &remove.remove_command {
        RemoveSubCommands::Card { id } => {
            Card::remove_card(id).await;
        }
        RemoveSubCommands::Webhook { all, id } => {
            Webhook::remove_webhook(*all, id.to_owned()).await;
        }
        RemoveSubCommands::Course { name } => {
            Card::remove_all_cards_by_labels(name.get_id()).await;
        }
    }
}
