use super::card::{Labels, Steps};
use crate::core::card::Card;
use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader};
use tokio;

#[derive(Deserialize, Debug)]
pub struct JsonCard {
    name: String,
    label: Labels,
    step: Steps,
}

pub async fn process_cads_from_json(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let cads: Vec<JsonCard> = from_reader(reader).unwrap();
    let mut tasks = vec![];

    for card in cads {
        let name = card.name;
        let label = card.label;
        let step = card.step;

        // Spawn 1 thread for each card creation
        let task = tokio::spawn(async move {
            Card::add_card(name, label, step).await
        });

        tasks.push(task);
    }

    for t in tasks {
        if let Err(e) = t.await {
            eprintln!("Erro Card Creation: {:?}", e);
        }
    }
}
