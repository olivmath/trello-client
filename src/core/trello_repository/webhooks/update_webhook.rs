use serde_json::json;

use crate::core::trello_repository::{
    utils::{get, put},
    webhooks::Webhook,
};
use std::env::var;

pub(crate) async fn update_webhook_by_id(id: &str, callback: &str, active: Option<bool>) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/webhooks/");
    base_url.push_str(id);

    let response = match get(base_url.as_str()).await {
        Ok(r) => {
            let status = r.status();
            let success = r.status().is_success();
            let response_text = r
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response body"));
            if success {
                println!("✅ Webhook Founded");
                match serde_json::from_str::<Webhook>(&response_text) {
                    Ok(w) => w,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {:?}", e);
                        eprintln!("Status code: {}, reason: {}", status, response_text);
                        panic!()
                    }
                }
            } else {
                println!("🚨 Webhook Not Found");
                println!("Status code: {}", status);
                println!("Response: {}", response_text);
                panic!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to connect");
            eprintln!("Error: {}", e);
            panic!();
        }
    };

    // update it
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let token = var("APIToken").expect("APIToken must be set");
    base_url.push_str("/tokens/");
    base_url.push_str(&token);
    base_url.push_str("/webhooks/");
    base_url.push_str(id);
    let body = json!({
        "callbackURL": callback,
        "description": "Update",
        "active": if active.is_some() { active.unwrap() } else {response.active},
    });

    match put(&base_url, body).await {
        Ok(r) => {
            let status = r.status();
            let success = r.status().is_success();
            let response_text = r
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response body"));
            if success {
                println!("✅ Webhook Updated");
            } else {
                println!("🚨 Webhook Updated failed");
                println!("Status code: {}", status);
                println!("Response: {}", response_text);
                panic!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to update webhook");
            eprintln!("Error: {}", e);
            panic!();
        }
    }
}
