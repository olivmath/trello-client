use std::env::var;

use crate::core::trello_repository::{labels::Labels, utils::get};

pub async fn get_all_labels() {
    // get all labels
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let board_id = var("BOARD_ID").expect("BOARD_ID must be set");

    base_url.push_str("/boards/");
    base_url.push_str(&board_id);
    base_url.push_str("/labels");

    let response = get(&base_url).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                println!("✅ Get all Labels");
                let text = r
                    .text()
                    .await
                    .unwrap_or_else(|_| "No response body".to_string());
                match serde_json::from_str::<Vec<Labels>>(&text) {
                    Ok(labels) => {
                        labels.iter().for_each(|c| println!("{}", c));
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

pub async fn get_all_labels_from_step(step_id: &str) {
    print!("GET ALL LABELS FROM STEP {}", step_id);
}
