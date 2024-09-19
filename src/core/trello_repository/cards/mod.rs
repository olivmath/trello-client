pub mod add_card;
pub mod delete_card;
pub mod get_card;

use crate::core::labels::Labels;
use crate::core::steps::Steps;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug)]
enum MoveCardOptions {
    Next,
    Back,
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    #[serde(rename = "shortUrl")]
    pub short_url: String,
    #[serde(rename = "idLabels")]
    pub id_labels: Vec<String>,
    #[serde(rename = "idList")]
    pub id_list: String,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut labels = String::new();
        self.id_labels.iter().for_each(|l| {
            if l.is_empty() {
                return labels.push_str("");
            }
            labels.push_str(&format!("{}, ", Labels::from_id(l)))
        });
        write!(
            f,
            "---------\nCard: {} \nCourse: {:?} \nSee here: {} \nStep: {:?} \n---------",
            self.name,
            labels,
            self.short_url,
            Steps::from_id(&self.id_list)
        )
    }
}

impl Card {
    pub async fn add_card(name: String, label: Labels, step: Steps) {
        add_card::add_card(&name, Labels::get_id(&label), Steps::get_id(&step)).await;
    }

    pub async fn get_card(
        all: bool,
        step: &Option<Steps>,
        label: &Option<Labels>,
        card_id: &Option<String>,
    ) {
        if all {
            get_card::get_all_cards().await;
        } else if let Some(step) = step {
            get_card::get_all_cards_from_step(step.get_id()).await;
        } else if let Some(label) = label {
            get_card::get_all_cards_from_label(label.get_id()).await;
        } else if let Some(id) = card_id {
            get_card::get_card(id.to_owned()).await;
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

    pub async fn remove_card(id: &str) {
        delete_card::delete_card_by_id(id).await;
    }

    pub async fn remove_all_cards_by_labels(labels: &str) {
        delete_card::delete_all_cards_from_labels(labels).await;
    }
}
