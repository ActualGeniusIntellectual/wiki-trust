mod init;
mod models;
mod revision;

fn main() {
    println!("Hello, world!");
    init::init();
    let revs = revision::all("Earth");

    // Pretty print the revision
    println!("{:#?}", revs);

    println!("Goodbye cruel world!");
}
