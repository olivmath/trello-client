use std::fmt::Display;

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

impl Steps {
    pub fn get_id(&self) -> &str {
        StepData::get_id(self)
    }
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

    pub fn get_step_by_id(id: &str) -> Steps {
        match id {
            "6633bf10887fb53e55941192" => Steps::Backlog,
            "6633a2169d3a4098e4adcd18" => Steps::PreProduction,
            "6633a20b86f043945dfaed69" => Steps::VisualIdentity,
            "6633a227d4871d117fa5fd87" => Steps::Recording,
            "6633a2399343322d437204b7" => Steps::Review,
            "6633bf1ce15c02de31b3797d" => Steps::Editing,
            "6633a24310b855a087d3713a" => Steps::Publish,
            "6633bf98240d7b4e3ec3b4bc" => Steps::Completed,
            _ => panic!("Step id not exist"),
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
    #[clap(alias = "w1")]
    Web1,
}

impl Display for Labels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Labels::ProductWeb3 => write!(f, "Product Web3"),
            Labels::BlockchainDevOps => write!(f, "Blockchain DevOps"),
            Labels::MachineLearning => write!(f, "Machine Learning"),
            Labels::Web1 => write!(f, "Web 1"),
        }
    }
}

impl Labels {
    pub fn get_id(&self) -> &str {
        match self {
            Labels::ProductWeb3 => "6633a8093462cf3f04d83047",
            Labels::BlockchainDevOps => "6633a612abab8af0d5ef1f8d",
            Labels::MachineLearning => "6633a8187180a66a1bdffb51",
            Labels::Web1 => "6633a5a8b5ccb9f6c72301b9",
        }
    }
}

pub struct LabelsData;

impl LabelsData {
    pub fn get_color_by_id(label: &str) -> &str {
        match label {
            "6633a8093462cf3f04d83047" => "yellow",
            "6633a612abab8af0d5ef1f8d" => "red",
            "6633a8187180a66a1bdffb51" => "purple",
            "6633a5a8b5ccb9f6c72301b9" => "green",
            _ => panic!("label id not exist"),
        }
    }
    pub fn get_id(label: &Labels) -> &str {
        match label {
            Labels::ProductWeb3 => "6633a8093462cf3f04d83047",
            Labels::BlockchainDevOps => "6633a612abab8af0d5ef1f8d",
            Labels::MachineLearning => "6633a8187180a66a1bdffb51",
            Labels::Web1 => "6633a5a8b5ccb9f6c72301b9",
        }
    }
    pub fn get_color(label: &Labels) -> &str {
        match label {
            Labels::ProductWeb3 => "yellow",
            Labels::BlockchainDevOps => "red",
            Labels::MachineLearning => "purple_dark",
            Labels::Web1 => "green",
        }
    }

    pub fn get_label_by_id(id: &str) -> Labels {
        match id {
            "6633a8093462cf3f04d83047" => Labels::ProductWeb3,
            "6633a612abab8af0d5ef1f8d" => Labels::BlockchainDevOps,
            "6633a8187180a66a1bdffb51" => Labels::MachineLearning,
            "6633a5a8b5ccb9f6c72301b9" => Labels::Web1,
            _ => panic!("label id not exist"),
        }
    }
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
        self.id_labels.iter().for_each(| l | {
            if l.is_empty() {
                return labels.push_str("");
            }
            labels.push_str(&format!("{}, ", LabelsData::get_label_by_id(l)))
        });
        write!(
            f,
            "---------\nCard: {} \nCourse: {:?} \nSee here: {} \nStep: {:?} \n---------",
            self.name,
            labels,
            self.short_url,
            StepData::get_step_by_id(&self.id_list)
        )
    }
}

impl Card {
    pub async fn add_card(name: String, label: Labels, step: Steps) {
        trello_reposirtory::add_card::add_card(
            &name,
            LabelsData::get_id(&label),
            StepData::get_id(&step),
        )
        .await;
    }

    pub async fn get_card(
        all: bool,
        step: &Option<Steps>,
        label: &Option<Labels>,
        card_id: &Option<String>,
    ) {
        if all {
            trello_reposirtory::get_card::get_all_cards().await;
        } else if let Some(step) = step {
            trello_reposirtory::get_card::get_all_cards_from_step(step.get_id()).await;
        } else if let Some(label) = label {
            trello_reposirtory::get_card::get_all_cards_from_label(label.get_id()).await;
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

    pub async fn remove_card(all: bool, card_id: &Option<String>) {
        if all {
            trello_reposirtory::delete_card::delete_all_cards().await;
        } else if let Some(id) = card_id {
            trello_reposirtory::delete_card::delete_card(id).await;
        }
    }
}
