use crate::core::trello_repository::utils::{delete, get};
use std::env::var;

use super::Webhook;

pub async fn delete_webhook_by_id(id: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let api_token = var("APIToken").expect("APIToken must be set");
    base_url.push_str("/tokens/");
    base_url.push_str(api_token.as_str());
    base_url.push_str("/webhooks/");
    base_url.push_str(id);

    match delete(base_url.as_str()).await {
        Ok(r) => {
            let status = r.status();
            let success = r.status().is_success();
            let response_text = r
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response body"));
            if success {
                println!("✅ Webhook Deleted: {}", id);
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

pub(crate) async fn delete_all_webhook() {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    let api_token = var("APIToken").expect("APIToken must be set");
    base_url.push_str("/tokens/");
    base_url.push_str(api_token.as_str());
    base_url.push_str("/webhooks");

    let response = get(base_url.as_str()).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                let text = r
                    .text()
                    .await
                    .unwrap_or_else(|_| "No response body".to_string());
                match serde_json::from_str::<Vec<Webhook>>(&text) {
                    Ok(webhooks) => {
                        let mut tasks = vec![];

                        for hook in webhooks {
                            println!("{}", hook);
                            let task = tokio::spawn(async move {
                                delete_webhook_by_id(&hook.id).await;
                            });
                            tasks.push(task);
                        }

                        for t in tasks {
                            if let Err(e) = t.await {
                                eprintln!("Erro Webhook Deleting: {:?}", e);
                            }
                        }
                        println!("✅ Deleted all Webhook");
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
            eprintln!("🚨 Get Webhook Error");
            eprintln!("{:?}", e);
            panic!()
        }
    }
}
