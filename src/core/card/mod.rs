use crate::core::trello_reposirtory;
use clap::ValueEnum;
use serde::Deserialize;

#[derive(Debug)]
enum MoveCardOptions {
    Next,
    Back,
}

#[derive(Deserialize, Hash, Eq, PartialEq, Debug, Clone, ValueEnum)]
pub enum Steps {
    Backlog,
    PreProduction,
    VisualIdentity,
    Recording,
    Review,
    Editing,
    Publish,
    Completed,
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
            Steps::Completed => "6633bf98240d7b4e3ec3b4bc",
        }
    }
}

#[derive(Deserialize, Hash, Eq, PartialEq, Debug, Clone, ValueEnum)]
pub enum Labels {
    #[clap(alias = "pw3")]
    ProductWeb3,
    #[clap(alias = "bdo")]
    BlockchainDevOps,
    #[clap(alias = "ml")]
    MachineLearning,
}

pub struct LabelsData;
impl LabelsData {
    pub fn get_color_by_id(label: &str) -> &str {
        match label {
            "6633a8093462cf3f04d83047" => "yellow",
            "6633a612abab8af0d5ef1f8d" => "red",
            "6633a8187180a66a1bdffb51" => "purple",
            _ => panic!("label id not exist"),
        }
    }
    pub fn get_id(label: &Labels) -> &str {
        match label {
            Labels::ProductWeb3 => "6633a8093462cf3f04d83047",
            Labels::BlockchainDevOps => "6633a612abab8af0d5ef1f8d",
            Labels::MachineLearning => "6633a8187180a66a1bdffb51",
        }
    }
    pub fn get_color(label: &Labels) -> &str {
        match label {
            Labels::ProductWeb3 => "yellow",
            Labels::BlockchainDevOps => "red",
            Labels::MachineLearning => "purple_dark",
        }
    }
}

pub struct Card {}

impl Card {
    pub async fn add_card(name: String, label: Labels, step: Steps) {
        trello_reposirtory::add_card::add_card(
            &name,
            LabelsData::get_id(&label),
            StepData::get_id(&step),
        )
        .await;
    }

    pub async fn get_card(all: bool, step: &Option<Steps>, card_id: &Option<String>) {
        if all {
            trello_reposirtory::get_card::get_all_cards().await;
        } else if let Some(step) = step {
            trello_reposirtory::get_card::get_all_cards_from_step(step.to_owned()).await;
        } else if let Some(id) = card_id {
            trello_reposirtory::get_card::get_card(id.to_owned()).await;
        }
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
