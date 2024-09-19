use crate::core::{
    labels::Labels,
    trello_repository::{cards::Card, utils::get},
};
use std::env::var;

pub async fn get_all_cards_from_label(label: &str) {
    // get all cards
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let board_id = var("BOARD_ID").expect("BOARD_ID must be set");

    base_url.push_str("/boards/");
    base_url.push_str(&board_id);
    base_url.push_str("/cards");

    let response = get(&base_url).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                println!("✅ Get all Card from Label: {:?}", Labels::from_id(label));
                let text = r
                    .text()
                    .await
                    .unwrap_or_else(|_| "No response body".to_string());
                match serde_json::from_str::<Vec<Card>>(&text) {
                    Ok(cards) => {
                        cards.iter().for_each(|c| println!("{}", c));
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

pub async fn get_all_cards_from_step(step: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");

    base_url.push_str("/lists/");
    base_url.push_str(step);
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
                        println!("✅ Get Cards from Step: {}", step);
                        for card in cards {
                            println!("{}", card);
                        }
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

pub async fn get_all_cards() {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let board_id = var("BOARD_ID").expect("BOARD_ID must be set");

    base_url.push_str("/boards/");
    base_url.push_str(&board_id);
    base_url.push_str("/cards");

    let response = get(&base_url).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                println!("✅ Get all Card");
                let text = r
                    .text()
                    .await
                    .unwrap_or_else(|_| "No response body".to_string());
                match serde_json::from_str::<Vec<Card>>(&text) {
                    Ok(cards) => {
                        for card in cards {
                            println!("{}", card);
                        }
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

pub async fn get_card(id: String) {
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
                match serde_json::from_str::<Card>(&text) {
                    Ok(card) => {
                        println!("{}", card);
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
