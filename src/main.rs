mod count;
mod init;
mod models;
mod revision;

use crate::models::Revision;
use crate::models::RevisionTable;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    init::init();
    let db = init::init_surreal().await;
    let page = "Arvin,_California".to_string();

    let count = count::retrieve(&page).await;
    log::info!("Revision count for page: {} is: {}", page, count);

    let revs = revision::retrieve(&page).await;

    // Store the count in the database
    for rev in revs {
        let rt: Result<Vec<Revision>, sqlite::Error> = db
            .create("revisions")
            .content(RevisionTable {
                page: page.clone(),
                revisions: rev.into(),
            })
            .await;

        match rt {
            Ok(_) => log::info!("Revision inserted"),
            Err(e) => log::error!("Error inserting revision: {:#?}", e),
        }
    }

    println!("Goodbye cruel world!");
}
