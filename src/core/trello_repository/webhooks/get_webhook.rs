use std::env::var;

use crate::core::trello_repository::utils::get;

pub async fn get_webhook_by_id(id: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/webhooks/");
    base_url.push_str(id);

    match get(base_url.as_str()).await {
        Ok(r) => {
            let status = r.status();
            let success = r.status().is_success();
            let response_text = r
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response body"));
            if success {
                println!("✅ Webhook Founded");
                println!("Status code: {}", status);
                println!("Response: {}", response_text);
            } else {
                println!("🚨 Webhook Not Found");
                println!("Status code: {}", status);
                println!("Response: {}", response_text);
                panic!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create webhook");
            eprintln!("Error: {}", e);
        }
    }
}

pub async fn get_all_webhook() {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let api_token = var("APIToken").expect("APIToken must be set");
    base_url.push_str("/tokens/");
    base_url.push_str(api_token.as_str());
    base_url.push_str("/webhooks");

    match get(base_url.as_str()).await {
        Ok(r) => {
            let status = r.status();
            let success = r.status().is_success();
            let response_text = r
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response body"));
            if success {
                println!("✅ Webhook Founded");
                println!("Status code: {}", status);
                println!("Response: {}", response_text);
            } else {
                println!("🚨 Webhook Not Found");
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
