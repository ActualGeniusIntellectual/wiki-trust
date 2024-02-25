// Add these to your Cargo.toml
// [dependencies]
// reqwest = "0.11"
// log = "0.4"
// env_logger = "0.9"
// rusqlite = { version = "0.26", features = ["bundled"] }
// serde_json = "1.0"

use env_logger::Builder;

use chrono::Local;
use log::{debug, info, LevelFilter};
use reqwest;
use rusqlite::{params, Connection, Result};
use serde_json::Value;
use std::io::Write;

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
        .filter(None, LevelFilter::Info)
        .init();

    let conn = Connection::open("revisions.db").expect("Error opening database.");

    debug!("Database connection established.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS content (
            revision_id INTEGER PRIMARY KEY,
            content TEXT,
            FOREIGN KEY(revision_id) REFERENCES revisions(id)
        )",
        [],
    )
    .expect("Error creating table.");

    info!("Database setup complete.");
    conn.close().expect("Error closing database.");
}

fn main() -> Result<()> {
    init();
    let conn = Connection::open("revisions.db")?;

    info!("Database setup complete.");

    process_revisions(&conn)?;

    conn.close().unwrap();
    info!("Database connection closed.");
    Ok(())
}

fn get_revision_content(rev_id: i64) -> Result<String, reqwest::Error> {
    debug!("Fetching content for revision ID: {}", rev_id);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(WIKI_API_URL)
        .query(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "revisions"),
            ("rvprop", "content"),
            ("revids", &rev_id.to_string()),
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
    let content = data["query"]["pages"][page_id]["revisions"][0]["*"]
        .as_str()
        .unwrap()
        .to_string();
    Ok(content)
}

fn store_content(conn: &Connection, rev_id: i64, content: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO content (revision_id, content) VALUES (?, ?)",
        params![rev_id, content],
    )?;
    Ok(())
}

fn process_revisions(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id FROM revisions WHERE id NOT IN (SELECT revision_id FROM content)")?;
    let revision_ids = stmt.query_map([], |row| row.get(0))?;

    for rev_id in revision_ids {
        let rev_id = rev_id?;
        match get_revision_content(rev_id) {
            Ok(content) => {
                store_content(conn, rev_id, &content)?;
                info!("{}", rev_id);
            }
            Err(e) => debug!("Error fetching content for revision ID {}: {}", rev_id, e),
        }
    }
    Ok(())
}
