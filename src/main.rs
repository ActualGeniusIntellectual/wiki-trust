use env_logger::Builder;

use chrono::Local;
use log::{debug, error, info, LevelFilter};
use rusqlite::{params, Connection, Result};
use std::io::Write;

static WIKI_API_URL: &str = "https://en.wikipedia.org/w/api.php";

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct Warning {
    #[serde(rename = "*")]
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct Warnings {
    main: Warning,
    revisions: Warning,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct Revision {
    contentformat: String,
    contentmodel: String,

    #[serde(rename = "*")]
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct Page {
    pageid: u32,
    ns: u8,
    title: String,
    revisions: Vec<Revision>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct Query {
    pages: HashMap<String, Page>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct ApiResponse {
    batchcomplete: String,
    warnings: Warnings,
    query: Query,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct Content {
    page: String,
    id: u32,
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
        .filter(None, LevelFilter::Info)
        .init();

    let conn = Connection::open("revisions.db").expect("Error opening database.");

    debug!("Database connection established.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS content (
            revision_id INTEGER PRIMARY KEY,
            page TEXT,
            content TEXT,
            FOREIGN KEY(revision_id) REFERENCES revisions(id),
            FOREIGN KEY(page) REFERENCES pages(title)
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

    process_revisions(&conn)?;

    conn.close().unwrap();
    info!("Database connection closed.");
    Ok(())
}

fn get_revision_content(rev_id: u32) -> Result<String, reqwest::Error> {
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
        .send();

    if let Err(e) = response {
        return Err(e);
    }

    let response = response.unwrap();

    // Debug log url
    debug!("URL: {}", response.url());

    let data: ApiResponse = response.json()?;
    debug!("Response: {:#?}", data);

    // Get the first revision
    let mut pages = data.query.pages.values();

    let first_page = match pages.next() {
        Some(page) => page,
        None => panic!("No pages found"),
    };

    let revisions = &first_page.revisions;
    let first_revision = match revisions.first() {
        Some(revision) => revision,
        None => panic!("No revisions found"),
    };

    let content = first_revision.content.clone();

    debug!("Content: {}", content);
    Ok(content)
}

fn store_content(conn: &Connection, rev_id: u32, page: String, content: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO content (revision_id, page, content) VALUES (?, ?, ?)",
        params![rev_id, page, content],
    )?;
    Ok(())
}

fn process_revisions(conn: &Connection) -> Result<()> {
    debug!("Processing revisions.");

    let mut stmt = conn.prepare(
    "SELECT page, id FROM revisions WHERE id NOT IN (SELECT revision_id FROM content) ORDER BY page",
    )?;

    // Return a list of revision IDs and their associated page
    let revs = stmt.query_map([], |row| {
        Ok(Content {
            page: row.get(0)?,
            id: row.get(1)?,
        })
    })?;

    // Convert into a list of Result<(String, u64)>
    let revs: Vec<Result<Content>> = revs.collect();

    info!("Revisions to process: {:#?}", revs.len());

    for rev_id in revs {
        let rev = rev_id?;

        match get_revision_content(rev.id) {
            Ok(content) => {
                info!("{} {}", rev.page, rev.id);
                store_content(conn, rev.id, rev.page, &content)?;
            }
            Err(e) => error!("Error: {} ({}): {}", rev.id, rev.page, e),
        }
    }
    Ok(())
}
