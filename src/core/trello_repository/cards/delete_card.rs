use super::super::utils::get;
use crate::core::labels::Labels;
use crate::core::{trello_repository::cards::Card, trello_repository::utils::delete};
use std::env::var;
use std::io::{self, Write};

pub async fn delete_card_by_id(id: &str) {
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

pub async fn delete_all_cards_from_labels(label: &str) {
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
                        let filtered_cards: Vec<Card> = cards
                            .into_iter()
                            .filter(|card| card.id_labels.iter().any(|l| l == label))
                            .collect();

                        println!(
                            "Found {} cards with the label '{}'",
                            filtered_cards.len(),
                            Labels::from_id(label)
                        );

                        print!("Do you want to delete all? (y/n): ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        let input = input.trim();

                        match input {
                            "y" => {
                                let mut tasks = vec![];
                                for card in filtered_cards {
                                    let task = tokio::task::spawn(async move {
                                        delete_card_by_id(&card.id).await;
                                    });
                                    tasks.push(task);
                                }

                                for t in tasks {
                                    if let Err(e) = t.await {
                                        eprintln!("Error deleting card: {:?}", e);
                                    }
                                }
                                println!("✅ Deleted all cards");
                            }
                            "n" => {
                                println!("These are the cards that would be deleted:");
                                for card in filtered_cards {
                                    println!("{}", card);
                                }
                            }
                            _ => {
                                println!("Invalid option. No cards were deleted.");
                            }
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
