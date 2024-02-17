mod count;
mod init;
mod models;
mod revision;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    init::init();
    let db = init::init_surreal().await;
    let page = "Earth".to_string();

    let count = count::retrieve(&page).await;
    println!("Revision count for page: {} is: {}", page, count);

    println!("Goodbye cruel world!");
}
