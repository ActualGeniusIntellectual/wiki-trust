// src/api.rs

use crate::models::{Revision, RevisionResponse};
use crate::store::{write_store, Store};

use reqwest::blocking::Client;

fn url(page: &str) -> String {
    log::debug!("Getting revision for page: {}", page);
    format!("https://en.wikipedia.org/w/rest.php/v1/page/{page}/history")
}

fn get(client: Client, page: &str) -> RevisionResponse {
    let url: String = url(page);
    let response = client.get(&url).send().unwrap();
    let body = response.text().unwrap();
    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision
}

fn next(client: Client, url: String) -> RevisionResponse {
    let response = client.get(&url).send().unwrap();
    let body = response.text().unwrap();

    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision
}

// Function to subsequently get the next revision page
pub fn all(cache: &mut Store, page: &str) -> Vec<Revision> {
    if let Some(revisions) = cache.revisions.get(page) {
        log::info!(
            "Cache hit for page: {} with {} revisions",
            page,
            revisions.len()
        );
        revisions.clone()
    } else {
        let client = reqwest::blocking::Client::new();

        let mut resp: RevisionResponse = get(client.clone(), page);
        let mut revisions: Vec<Revision> = Vec::new();
        revisions.extend(resp.revisions);

        while let Some(older) = resp.older {
            resp = next(client.clone(), older);
            revisions.extend(resp.revisions);
        }

        // Update the cache
        write_store(cache, page, revisions.clone());
        revisions
    }
}
