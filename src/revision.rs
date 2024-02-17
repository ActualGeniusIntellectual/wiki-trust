// src/api.rs

use crate::models::{Revision, RevisionResponse};

fn url(page: &str) -> String {
    log::info!("Getting revision for page: {}", page);
    format!("https://en.wikipedia.org/w/rest.php/v1/page/{page}/history")
}

fn get(page: &str) -> RevisionResponse {
    let url: String = url(page);
    let response = reqwest::blocking::get(&url).unwrap();
    let body = response.text().unwrap();
    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision
}

fn next(url: String) -> RevisionResponse {
    let response = reqwest::blocking::get(&url).unwrap();
    let body = response.text().unwrap();
    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    for rev in &revision.revisions {
        log::debug!("Revision: {:?}", rev);
    }

    revision
}

// Function to subsequently get the next revision page
pub fn all(page: &str) -> Vec<Revision> {
    let mut resp: RevisionResponse = get(page);
    let mut revisions: Vec<Revision> = Vec::new();
    revisions.extend(resp.revisions);

    while let Some(older) = resp.older {
        resp = next(older);
        revisions.extend(resp.revisions);
    }

    revisions
}
