use crate::core::{card::Card, trello_reposirtory::utils::delete};
use std::env::var;

use super::utils::get;

pub async fn delete_card(id: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/cards/");
    base_url.push_str(id);

    let response = delete(&base_url).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                println!("✅ Deleted Card: {}", id);
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

pub async fn delete_all_cards() {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let board_id = var("BOARD_ID").expect("BOARD_ID must be set");

    base_url.push_str("/boards/");
    base_url.push_str(&board_id);
    base_url.push_str("/cards");

    let response = get(&base_url).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                let text = r
                    .text()
                    .await
                    .unwrap_or_else(|_| "No response body".to_string());
                match serde_json::from_str::<Vec<Card>>(&text) {
                    Ok(cards) => {
                        let mut tasks = vec![];

                        for card in cards {
                            println!("{}", card);
                            let task = tokio::spawn(async move {
                                delete_card(&card.id).await;
                            });
                            tasks.push(task);
                        }

                        for t in tasks {
                            if let Err(e) = t.await {
                                eprintln!("Erro Card Creation: {:?}", e);
                            }
                        }
                        println!("✅ Deleted all Card");
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
