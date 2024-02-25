use chrono::Local;
use env_logger::Builder;
use log::{debug, error, info, LevelFilter};
use rayon::prelude::*;
use reqwest;
use rusqlite::{params, Connection, Result};
use serde_json::Value;
use std::io::Write;

mod lists;
use lists::build_page_titles;

static WIKI_API_URL: &str = "https://en.wikipedia.org/w/api.php";

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
    conn.close().unwrap();
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

fn fetch_and_store_revisions() -> Result<()> {
    let pages = build_page_titles();

    pages.par_iter().for_each(|&page_title| {
        let conn = Connection::open("revisions.db").unwrap();
        let stored_revisions_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM revisions WHERE page = ?",
                [page_title],
                |row| row.get(0),
            )
            .unwrap();
        info!(
            "Checking stored revisions for {}: {} revisions found.",
            page_title, stored_revisions_count
        );

        let api_revisions_count = get_revision_count(page_title).unwrap();
        debug!(
            "API revisions count for {}: {}",
            page_title, api_revisions_count
        );

        if stored_revisions_count < api_revisions_count as i64 {
            debug!("Fetching new revisions for {}.", page_title);
            // More detailed implementation for fetching and storing revisions goes here
            // Similar to the Python script but using Rust's reqwest and rusqlite libraries
        }
    });

    Ok(())
}
