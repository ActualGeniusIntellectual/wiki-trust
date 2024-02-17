use serde::{Deserialize, Serialize};

// Embed .store.json file into the binary
const _STORE: &str = include_str!("../.store.json");

// Struct for the store
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
struct Store {
    pages: Vec<Page>,
}

// Struct for the page
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
struct Page {
    title: String,
    content: String,
}

// --------------------------------

// Struct for the response from the API
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
struct RevisionResponse {
    revisions: Vec<Revision>,
    newer: Option<String>,
    older: Option<String>,
    latest: Option<String>,
}

// Struct for the user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
struct Revision {
    id: u64,
    timestamp: String,
    minor: bool,
    size: u32,
    comment: String,
    user: User,
    delta: i32,
}

// Parse the .store.json file into a Store struct
// Make function const so it can be used in const context
//fn parse_store() -> Store {
// Parse the .store.json file into a Store struct
//    serde_json::from_str(&STORE).unwrap()
//}

// Init store
//fn init_store() -> Store {
//    parse_store()
//}

fn init_dotenv() {
    use dotenv::dotenv;

    dotenv().ok().expect("Failed to read .env file");
}

fn init_log() {
    use chrono::Local;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::io::Write;

    rayon::ThreadPoolBuilder::new()
        .num_threads(32)
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

    log::info!("Logger initialized");
}

fn revision_url(page: &str) -> String {
    log::info!("Getting revision for page: {}", page);
    format!("https://en.wikipedia.org/w/rest.php/v1/page/{page}/history")
}

fn revision(page: &str) -> RevisionResponse {
    let url: String = revision_url(page);
    let response = reqwest::blocking::get(&url).unwrap();
    let body = response.text().unwrap();
    let revision: RevisionResponse = serde_json::from_str(&body).unwrap();

    revision
}

fn init() {
    init_dotenv();
    init_log();
    //init_store()
}
fn main() {
    println!("Hello, world!");
    init();
    let revs = revision("Earth");

    // Pretty print the revision
    println!("{:#?}", revs);

    println!("Goodbye cruel world!");
}
