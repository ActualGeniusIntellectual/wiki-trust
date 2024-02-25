use chrono::Local;
use env_logger::Builder;
use log::{debug, error, info, LevelFilter};
use rayon::prelude::*;
use reqwest;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use serde_json::Value;
use std::io::Write;

mod lists;
use lists::build_page_titles;

static WIKI_API_URL: &str = "https://en.wikipedia.org/w/api.php";

// Create Serde struct for the revision count API response
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RevisionCount {
    count: u32,
    limit: bool,
}

fn init() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();

    // Initialize logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let conn = Connection::open("revisions.db").expect("Error opening database.");

    debug!("Database connection established.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS revisions (
            id INTEGER PRIMARY KEY,
            page TEXT,
            timestamp TEXT,
            minor BOOLEAN,
            size INTEGER,
            comment TEXT,
            user TEXT
        )",
        [],
    )
    .expect("Error creating table.");

    info!("Database setup complete.");
    conn.close().expect("Error closing database.");
}

fn main() -> Result<()> {
    init();
    info!("Starting...");
    debug!("Logger initialized.");

    debug!("Starting fetch_and_store_revisions.");
    fetch_and_store_revisions()?;

    info!("All done.");
    Ok(())
}

fn get_revision_count(page_title: &str) -> Result<u32, reqwest::Error> {
    let count_api_url = format!(
        "https://api.wikimedia.org/core/v1/wikipedia/en/page/{page_title}/history/counts/edits?"
    );

    debug!("Fetching revision count for {}.", page_title);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(count_api_url)
        .send()
        .expect("Error sending request.");

    // Use serde to parse the JSON response
    let revision_count: RevisionCount = response.json().expect("Error parsing JSON response.");

    debug!(
        "Revision count for {}: {}",
        page_title, revision_count.count
    );
    Ok(revision_count.count)
}

fn fetch_and_store_revisions() -> Result<()> {
    let pages = build_page_titles();

    pages.par_iter().for_each(|&page_title| {
        let conn = Connection::open("revisions.db").unwrap();

        let error_message = format!("Error fetching revisions for {}", page_title);
        let stored_revisions_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM revisions WHERE page = ?",
                [page_title],
                |row| row.get(0),
            )
            .expect(error_message.as_str());

        info!(
            "Checking stored revisions for {}: {} revisions found.",
            page_title, stored_revisions_count
        );

        let api_revisions_count =
            get_revision_count(page_title).expect("Error fetching revision count from API.");
        debug!(
            "API revisions count for {}: {}",
            page_title, api_revisions_count
        );

        if stored_revisions_count < api_revisions_count as i64 {
            debug!("Fetching new revisions for {}.", page_title);
        }

        conn.close().expect("Error closing database.");
    });

    Ok(())
}
