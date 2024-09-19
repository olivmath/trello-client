use crate::core::trello_repository::utils::post;
use core::panic;
use serde_json::json;
use std::env::var;

pub(crate) async fn add_webhook(name: &str, callback: &str) {
    let body = json!({
        "callbackURL": callback,
        "idModel": var("BOARD_ID").expect("BOARD_ID should be set"),
        "description": name,
    });

    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let token = var("APIToken").expect("APIToken must be set");
    base_url.push_str("/tokens/");
    base_url.push_str(&token);
    base_url.push_str("/webhooks");

    match post(&base_url, body).await {
        Ok(r) => {
            let status = r.status();
            let success = r.status().is_success();
            let response_text = r
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response body"));
            if success {
                println!("✅ Webhook created");
                println!("Status code: {}", status);
                println!("Response: {}", response_text);
            } else {
                println!("🚨 Failed Webhook creation");
                println!("Status code: {}", status);
                println!("Response: {}", response_text);
                panic!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create webhook");
            eprintln!("Error: {}", e);
            panic!();
        }
    }
}
