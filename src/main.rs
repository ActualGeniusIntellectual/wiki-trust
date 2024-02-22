// Add these to your Cargo.toml
// [dependencies]
// reqwest = "0.11"
// log = "0.4"
// env_logger = "0.9"
// rusqlite = { version = "0.26", features = ["bundled"] }
// serde_json = "1.0"

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use log::{debug, error, info};
use reqwest;
use rusqlite::{params, Connection, Result};
use serde_json::Value;
use std::io::Write;

static WIKI_API_URL: &str = "https://en.wikipedia.org/w/api.php";
const PAGE_TITLES: &[&str] = &["United_States"]; // Add your page titles

fn init() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(rayon::current_num_threads())
        .build_global()
        .unwrap();

    // Initialize logger
    Builder::new().format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    });
}

fn main() -> Result<()> {
    init();
    let conn = Connection::open("revisions.db")?;

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
    )?;
    info!("Database setup complete.");

    fetch_and_store_revisions(&conn)?;

    conn.close().unwrap();
    info!("Database connection closed.");
    Ok(())
}

fn get_revision_count(page_title: &str) -> Result<usize, reqwest::Error> {
    debug!("Fetching revision count for {}.", page_title);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(WIKI_API_URL)
        .query(&[
            ("action", "query"),
            ("format", "json"),
            ("titles", page_title),
            ("prop", "revisions"),
            ("rvprop", "ids"),
            ("rvlimit", "1"),
        ])
        .send()?;

    let data = response.json::<Value>()?;
    let page_id = data["query"]["pages"]
        .as_object()
        .unwrap()
        .keys()
        .next()
        .unwrap()
        .clone();
    let revision_count = data["query"]["pages"][page_id]["revisions"]
        .as_array()
        .unwrap()
        .len();
    debug!("Revision count for {}: {}", page_title, revision_count);
    Ok(revision_count)
}

fn fetch_and_store_revisions(conn: &Connection) -> Result<()> {
    for &page_title in PAGE_TITLES.iter() {
        let stored_revisions_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM revisions WHERE page = ?",
            [page_title],
            |row| row.get(0),
        )?;
        info!(
            "Checking stored revisions for {}: {} revisions found.",
            page_title, stored_revisions_count
        );

        let api_revisions_count = get_revision_count(page_title).unwrap();

        if stored_revisions_count < api_revisions_count as i64 {
            // More detailed implementation for fetching and storing revisions goes here
            // Similar to the Python script but using Rust's reqwest and rusqlite libraries
        }
    }
    Ok(())
}
