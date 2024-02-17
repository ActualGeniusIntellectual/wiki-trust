// src/init.rs
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

const NS: &str = "wikipedia";
const DB: &str = "revisions";

pub async fn init_surreal() -> Surreal<Client> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    // Select a specific namespace / database
    db.use_ns(NS).use_db(DB).await.unwrap();

    // Return the database
    db
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

pub fn init() {
    init_dotenv();
    init_log();
}
