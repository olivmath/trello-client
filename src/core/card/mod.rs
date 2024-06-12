use crate::core::trello_reposirtory::Trello;
use clap::ValueEnum;

#[derive(Debug)]
enum MoveCardOptions {
    Next,
    Back,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, ValueEnum)]
pub enum Steps {
    Backlog,
    PreProduction,
    VisualIdentity,
    Recording,
    Review,
    Editing,
    Publish,
    Complete,
}

pub struct StepData;

impl StepData {
    pub fn get_id(step: &Steps) -> &str {
        match step {
            Steps::Backlog => "6633bf10887fb53e55941192",
            Steps::PreProduction => "6633a2169d3a4098e4adcd18",
            Steps::VisualIdentity => "6633a20b86f043945dfaed69",
            Steps::Recording => "6633a227d4871d117fa5fd87",
            Steps::Review => "6633a2399343322d437204b7",
            Steps::Editing => "6633bf1ce15c02de31b3797d",
            Steps::Publish => "6633a24310b855a087d3713a",
            Steps::Complete => "6633bf98240d7b4e3ec3b4bc",
        }
    }
}

pub struct Card {}

impl Card {
    pub async fn add_card(name: &str, label: &str, step: &Steps) {
        Trello::add_card(name, label, StepData::get_id(step)).await;
    }

    pub fn move_card(my_board: &str, next: bool, back: bool, card_id: &str) {
        if next {
            println!(
                "Moving card with ID '{}' on board '{}' to the next step",
                card_id, my_board
            );
            println!("{:?}", MoveCardOptions::Next);
        } else if back {
            println!(
                "Moving card with ID '{}' on board '{}' to the previous step",
                card_id, my_board
            );
            println!("{:?}", MoveCardOptions::Back);
        }
    }

    pub fn edit_card(
        my_board: &str,
        card_id: &str,
        name: Option<&str>,
        label: Option<&str>,
        color: Option<&str>,
    ) {
        println!(
            "Editing card with ID '{}' on board '{}': Name = {:?}, Label = {:?}, Color = {:?}",
            card_id, my_board, name, label, color
        );
    }

    pub fn get_card(
        my_board: &str,
        all: bool,
        step: Option<&str>,
        card_name: Option<&str>,
        card_id: Option<&str>,
    ) {
        if all {
            println!("Getting all cards from board '{}'", my_board);
            // println!("{:?}", GetCardOptions::All);
        } else if let Some(step) = step {
            println!(
                "Getting all cards from board '{}' in step '{}'",
                my_board, step
            );
            // println!("{:?}", GetCardOptions::Step(step.to_string()));
        } else if let Some(card_name) = card_name {
            println!("Getting card '{}' from board '{}'", card_name, my_board);
            // println!("{:?}", GetCardOptions::CardName(card_name.to_string()));
        } else if let Some(card_id) = card_id {
            println!(
                "Getting card with ID '{}' from board '{}'",
                card_id, my_board
            );
            // println!("{:?}", GetCardOptions::CardId(card_id.to_string()));
        }
    }

    pub fn remove_card(my_board: &str, card_name: Option<&str>, card_id: Option<&str>) {
        if let Some(card_name) = card_name {
            println!("Removing card '{}' from board '{}'", card_name, my_board);
            // println!("{:?}", RemoveCardOptions::CardName(card_name.to_string()));
        } else if let Some(card_id) = card_id {
            println!(
                "Removing card with ID '{}' from board '{}'",
                card_id, my_board
            );
            // println!("{:?}", RemoveCardOptions::CardId(card_id.to_string()));
        }
    }
}
