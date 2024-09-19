pub mod get_labels;

use std::fmt::Display;

use crate::core::steps::Steps;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Labels {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(rename = "idBoard")]
    pub id_board: String,
}

impl Labels {
    pub async fn get_labels(all: bool, step: &Option<Steps>) {
        if all {
            get_labels::get_all_labels().await;
        } else if let Some(step) = step {
            get_labels::get_all_labels_from_step(step.get_id()).await;
        }
    }
}

impl Display for Labels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.id)
    }
}
