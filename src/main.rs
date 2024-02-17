mod api;
mod init;
mod models;

fn main() {
    println!("Hello, world!");
    init::init();
    let revs = api::revision("Earth");

    // Pretty print the revision
    println!("{:#?}", revs);

    println!("Goodbye cruel world!");
}
