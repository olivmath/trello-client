use super::super::utils::{post, put};
use crate::core::{labels::Labels, trello_repository::cards::Card};
use serde_json::json;
use std::env::var;

pub(crate) async fn add_card(name: &str, label: &str, step: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/cards");

    let body = json!({
        "idCardSource": var("TEMPLATE_CARD_ID").expect("TEMPLATE_CARD_ID must be set"),
        "idList": step,
        "idLabels": vec![label],
        "name": name,
    });

    let (card_id, short_url) = match post(&base_url, body).await {
        Ok(r) => {
            let status_code = r.status();
            let text = r
                .text()
                .await
                .unwrap_or_else(|_| "No response body".to_string());
            match serde_json::from_str::<Card>(&text) {
                Ok(Card { id, short_url, .. }) => (id, short_url),
                Err(e) => {
                    eprintln!("Failed to parse JSON: {:?}", e);
                    eprintln!("Status code: {}, reason: {}", status_code, text);
                    panic!()
                }
            }
        }

        Err(e) => {
            eprintln!("🚨 Create Card Error");
            eprintln!("Status code: {:?}", e.status());
            eprintln!("{:?}", e);
            panic!()
        }
    };

    edit_card(&card_id, label, step).await;
    println!("✅ Card created, id: {}", card_id);
    println!("🔗 See here: {}", short_url);
}

async fn edit_card(card_id: &str, label: &str, step: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/cards/");
    base_url.push_str(card_id);

    let color = Labels::get_color_by_id(label);
    let body = json!({
        "idCardSource": var("TEMPLATE_CARD_ID").expect("TEMPLATE_CARD_ID must be set"),
        "idList": step,
        "idLabels": vec![label],
        "cover": {
            "color": color,
            "size": "full"
        }
    });

    if let Err(e) = put(&base_url, body).await {
        eprintln!("🚨 Update Card Error");
        eprintln!("Status code: {:?}", e.status());
        eprintln!("{:?}", e);
        panic!()
    };
}
