// src/api.rs

use crate::models::{Revision, RevisionResponse};
use surrealdb::engine::remote::ws::{Client, Ws};

use reqwest::Client as ReqwestClient;

fn url(page: &str) -> String {
    log::debug!("Getting revision for page: {}", page);
    format!("https://en.wikipedia.org/w/rest.php/v1/page/{page}/history")
}

async fn get(client: ReqwestClient, page: &str) -> RevisionResponse {
    let url: String = url(page);
    let response = client.get(&url).send().await.unwrap();
    let body = response.text().await.unwrap();
    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision
}

async fn next(client: ReqwestClient, url: String) -> RevisionResponse {
    let response = client.get(&url).send().await.unwrap();
    let body = response.text().await.unwrap();

    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision
}

// Function to subsequently get the next revision page
pub async fn retrieve(page: &String) -> Vec<Revision> {
    log::info!("Getting all revisions for page: {}", page);

    let client = reqwest::Client::new();

    let mut resp: RevisionResponse = get(client.clone(), page).await;
    let mut revisions: Vec<Revision> = Vec::new();
    revisions.extend(resp.revisions);

    while let Some(older) = resp.older {
        resp = next(client.clone(), older).await;
        revisions.extend(resp.revisions);
    }

    revisions
}
