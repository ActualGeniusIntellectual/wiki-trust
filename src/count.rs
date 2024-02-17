// src/revision_count.rs

use crate::models::RevisionResponse;
use reqwest::Client as ReqwestClient;

fn url(page: &str) -> String {
    log::debug!("Getting revision count for page: {}", page);
    format!("https://en.wikipedia.org/w/rest.php/v1/page/{page}/history")
}

async fn get_revision_count(client: ReqwestClient, page: &str) -> usize {
    let url: String = url(page);
    let response = client.get(&url).send().await.unwrap();
    let body = response.text().await.unwrap();
    let revision_response: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision_response.revisions.len()
}

pub async fn retrieve(page: &String) -> usize {
    log::info!("Getting revision count for page: {}", page);

    let client = reqwest::Client::new();
    let revision_count = get_revision_count(client, page).await;

    revision_count
}
