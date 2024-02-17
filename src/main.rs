mod init;
mod models;
mod revision;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    init::init();
    let db = init::init_surreal().await;
    let page = "Earth".to_string();
    let revs = revision::retrieve(&page);

    // Insert the revisions into the database under the key "Earth"
    for data in revs.await {
        let revs: Vec<models::Revision> = db.create(&page).content(data).await.unwrap();
        log::info!("Inserted {} revisions", revs.len());
    }

    println!("Goodbye cruel world!");
}
