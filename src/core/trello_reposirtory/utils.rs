use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Response,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env::var};

#[derive(Deserialize, Debug)]
pub struct CardResponse {
    id: String,
    #[serde(rename = "shortUrl")]
    short_url: String,
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers
}

fn base_query<'a>() -> HashMap<&'a str, String> {
    let mut query = HashMap::new();
    let key = var("API_KEY").expect("API_KEY must be set");
    let token = var("TOKEN").expect("TOKEN must be set");

    query.insert("key", key);
    query.insert("token", token);

    query
}

fn get_client() -> Client {
    let headers = get_headers();

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_default()
}

async fn handle_response(response: Result<Response, Error>, card_id: Option<&mut String>) {
    match response {
        Ok(r) => {
            let text = r
                .text()
                .await
                .unwrap_or_else(|_| "No response body".to_string());
            match serde_json::from_str::<CardResponse>(&text) {
                Ok(card_response) => {
                    println!(
                        "✅ Card created with success, card id: {}",
                        card_response.id
                    );
                    println!("🔗 See here: {}", card_response.short_url);
                    *card_id.unwrap() = card_response.id;
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {:?}", e);
                    eprintln!("{}", text);
                }
            }
        }

        Err(e) => {
            eprintln!("🚨 Create card Error");
            eprintln!("Status code: {:?}", e.status());
            eprintln!("{:?}", e);
        }
    }
}

pub(super) async fn post<T: Serialize>(base_url: &str, body: T, card_id: Option<&mut String>) {
    let client = get_client();
    let query = base_query();

    let response = client.post(base_url).query(&query).json(&body).send().await;

    handle_response(response, card_id).await
}

pub(super) async fn put<T: Serialize>(base_url: &str, body: T, card_id: Option<&mut String>) {
    let client = get_client();
    let query = base_query();

    let response = client.put(base_url).query(&query).json(&body).send().await;

    handle_response(response, card_id).await
}

// pub(super) fn get() {}
// pub(super) fn delete() {}
