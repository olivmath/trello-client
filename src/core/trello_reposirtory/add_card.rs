use super::utils::post;
use std::{collections::HashMap, env::var};

pub(super) async fn add_card(name: &str, label: &str, step: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/cards");

    let mut body = HashMap::new();
    body.insert("idList", step);
    body.insert("label", label);
    body.insert("name", name);

    post(&base_url, body).await;
}
