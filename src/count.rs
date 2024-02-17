// src/revision_count.rs

use crate::models::CountResponse;
use reqwest::Client as ReqwestClient;

fn url(page: &str) -> String {
    log::debug!("Getting revision count for page: {}", page);
    format!("https://api.wikimedia.org/core/v1/wikipedia/en/page/{page}/history/counts/edits")
}

async fn get_revision_count(client: ReqwestClient, page: &str) -> u64 {
    let url: String = url(page);
    let response = client.get(&url).send().await.unwrap();
    let body = response.text().await.unwrap();
    let revision_response: CountResponse = serde_json::from_str(&body).unwrap();

    revision_response.count
}

pub async fn retrieve(page: &String) -> u64 {
    log::info!("Getting revision count for page: {}", page);

    let client = reqwest::Client::new();
    let revision_count = get_revision_count(client, page).await;

    revision_count
}
