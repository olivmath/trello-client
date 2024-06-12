use super::card::{Labels, Steps};
use crate::core::card::Card;
use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct JsonCard {
    name: String,
    label: Labels,
    step: Steps,
}

pub async fn process_cads_from_json(file_path: &str) {
    // Abre o arquivo JSON
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    // Deserializa o JSON em uma lista de `Class`
    let cads: Vec<JsonCard> = from_reader(reader).unwrap();

    // Itera sobre cada `Class` e chama `add_card`
    for class in cads {
        println!(
            "name: {}, label: {}, step: {}",
            &class.name,
            &format!("{:?}", class.label),
            &format!("{:?}", class.step)
        );
        Card::add_card(&class.name, &class.label, &class.step).await;
    }
}
