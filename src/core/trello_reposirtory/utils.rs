use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Response,
};
use serde_json::Value;
use std::{collections::HashMap, env::var};

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

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_default();

    client
}

async fn handle_response(response: Result<Response, Error>) {
    match response {
        Ok(r) => {
            let json: Result<Value, _> = r.json().await;
            match json {
                Ok(value) => {
                    println!("✅ Card created with success!");
                    if let Some(short_url) = value["shortUrl"].as_str() {
                        println!("🔗 See here: {}", short_url);
                    } else {
                        eprintln!("😢 shortUrl not found in the response, please open your board");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("🚨 Create card Error");
            eprintln!("Status code: {:?}", e.status());
            eprintln!("{:?}", e)
        }
    }
}

pub(super) async fn post(base_url: &str, body: HashMap<&str, &str>) {
    let client = get_client();
    let query = base_query();

    let response = client.post(base_url).query(&query).json(&body).send().await;

    handle_response(response).await
}

// pub(super) fn get() {}
// pub(super) fn put() {}
// pub(super) fn delete() {}
