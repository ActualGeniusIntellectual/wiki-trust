// src/api.rs

use crate::models::{Revision, RevisionResponse};

pub fn revision_url(page: &str) -> String {
    log::info!("Getting revision for page: {}", page);
    format!("https://en.wikipedia.org/w/rest.php/v1/page/{page}/history")
}

pub fn revision(page: &str) -> RevisionResponse {
    let url: String = revision_url(page);
    let response = reqwest::blocking::get(&url).unwrap();
    let body = response.text().unwrap();
    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision
}
