// src/init.rs

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

pub fn init() {
    init_dotenv();
    init_log();
}
