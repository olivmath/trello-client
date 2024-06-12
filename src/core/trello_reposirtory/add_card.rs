use super::utils::post;
use crate::core::trello_reposirtory::edit_card::edit_card;
use serde_json::json;
use std::env::var;

pub(super) async fn add_card(name: &str, label: &str, step: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/cards");

    let body = json!({
        "idCardSource": var("TEMPLATE_CARD_ID").expect("TEMPLATE_CARD_ID must be set"),
        "idList": step,
        "idLabels": vec![label],
        "name": name,
    });

    // punk approach
    // Pass-by-Mutable-Reference
    let mut card_id = String::new();
    post(&base_url, body, Some(&mut card_id)).await;

    // edit card
    edit_card(&card_id, label, step).await;
}
