mod init;
mod models;
mod revision;

mod store;

fn main() {
    println!("Hello, world!");
    init::init();
    let mut cache = store::init_store();
    let revs = revision::all(&mut cache, "Earth");

    println!("{:?}", revs);

    println!("Goodbye cruel world!");
}
