use serde::{Deserialize, Serialize};
use serde_json::Result;

// Embed .store.json file into the binary
const STORE: &str = include_str!("../.store.json");

// Struct for the store
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Store {
    revision: Vec<Revision>,
}

// Struct for the revision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Revision {
    id: u64,
    timestamp: u64,
    data: u64,
}

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
        .filter(None, LevelFilter::Info)
        .init();

    log::info!("Logger initialized");
}

fn init() {
    init_dotenv();
    init_log();
}
fn main() {
    init();
    println!("Hello, world!");
    println!("Goodbye cruel world!");
}
