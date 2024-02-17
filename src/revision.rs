// src/api.rs

use crate::models::{Revision, RevisionResponse};
use reqwest::blocking::Client;

fn url(page: &str) -> String {
    log::info!("Getting revision for page: {}", page);
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

    for rev in &revision.revisions {
        // Only print if any of the fields are None
        if rev.id.is_none()
            || rev.timestamp.is_none()
            || rev.minor.is_none()
            || rev.size.is_none()
            || rev.comment.is_none()
            || rev.user.is_none()
            || rev.delta.is_none()
        {
            log::warn!("{:?}", rev);
        }
    }

    revision
}

// Function to subsequently get the next revision page
pub fn all(page: &str) -> Vec<Revision> {
    let client = reqwest::blocking::Client::new();

    let mut resp: RevisionResponse = get(client.clone(), page);
    let mut revisions: Vec<Revision> = Vec::new();
    revisions.extend(resp.revisions);

    while let Some(older) = resp.older {
        resp = next(client.clone(), older);
        revisions.extend(resp.revisions);
    }

    revisions
}
