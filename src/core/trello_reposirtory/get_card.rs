use crate::core::{
    card::{LabelsData, StepData, Steps},
    trello_reposirtory::utils::get,
};
use serde::Deserialize;
use std::env::var;

#[derive(Deserialize, Debug)]
struct CardResponse {
    name: String,
    #[serde(rename = "shortUrl")]
    short_url: String,
    #[serde(rename = "idLabels")]
    id_labels: Vec<String>,
    #[serde(rename = "idList")]
    id_list: String,
}

pub async fn get_all_cards_from_step(step: Steps) {
    println!("get_all_cards_from_step: {:?}", step);
}
pub async fn get_all_cards() {
    println!("get_all_cards");
}
pub async fn get_card(id: String) {
    println!("get_card: {}", id);

    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/cards/");
    base_url.push_str(id.as_str());

    let response = get(&base_url).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                println!("✅ Get Card: {}", id);
                let text = r
                    .text()
                    .await
                    .unwrap_or_else(|_| "No response body".to_string());
                match serde_json::from_str::<CardResponse>(&text) {
                    Ok(CardResponse {
                        name,
                        short_url,
                        id_list,
                        id_labels,
                        ..
                    }) => {
                        println!("Card: {}", name);
                        println!("See here: {}", short_url);
                        println!("Step: {:?}", StepData::get_step_by_id(&id_list));
                        println!("Label: {:?}", LabelsData::get_label_by_id(&id_labels[0]));
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {:?}", e);
                        eprintln!("{}", text);
                        panic!()
                    }
                }
            } else {
                eprintln!("Error: Received status code {}", r.status());
                eprintln!("Response: {}", r.text().await.unwrap());
            }
        }
        Err(e) => {
            eprintln!("🚨 Get Card Error");
            eprintln!("{:?}", e);
            panic!()
        }
    }
}
