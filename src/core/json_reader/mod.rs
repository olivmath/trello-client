use super::labels::Labels;
use super::steps::Steps;
use crate::core::trello_repository::cards::Card;
use serde::Deserialize;
use serde_json::from_reader;
use std::{env::var, fs::File, io::BufReader};
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

    println!("✅ Add {} cards", cads.len());

    let rate_limit = var("RATE_LIMIT")
        .expect("RATE_LIMIT must be set")
        .parse::<usize>()
        .unwrap();
    for chunk in cads.chunks(rate_limit) {
        let mut tasks = vec![];

        for card in chunk {
            let name = card.name.clone();
            let label = card.label;
            let step = card.step;

            let task = tokio::spawn(async move { Card::add_card(name, label, step).await });

            tasks.push(task);
        }

        for task in tasks {
            if let Err(e) = task.await {
                eprintln!("Erro Card Creation: {:?}", e);
            }
        }
    }
}
