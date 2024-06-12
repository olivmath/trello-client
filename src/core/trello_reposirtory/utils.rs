use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Response,
};
use serde::Serialize;
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

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_default()
}

pub(super) async fn post<T: Serialize>(base_url: &str, body: T) -> Result<Response, Error> {
    let client = get_client();
    let query = base_query();

    client.post(base_url).query(&query).json(&body).send().await
}

pub(super) async fn put<T: Serialize>(base_url: &str, body: T) -> Result<Response, Error> {
    let client = get_client();
    let query = base_query();

    client.put(base_url).query(&query).json(&body).send().await
}

// pub(super) fn get() {}
// pub(super) fn delete() {}
