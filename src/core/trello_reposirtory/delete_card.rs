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

