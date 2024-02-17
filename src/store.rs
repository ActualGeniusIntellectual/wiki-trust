use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::Revision;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Store {
    pub revisions: HashMap<String, Vec<Revision>>,
}

const STORE_STR: &str = include_str!(".././.store.json");

pub fn init_store() -> Store {
    serde_json::from_str(STORE_STR).unwrap()
}

// Write new lines to the store
pub fn write_store(store: &mut Store, page: &str, revisions: Vec<Revision>) {
    store.revisions.insert(page.to_string(), revisions);
    write_store_to_disk(store);
}

// Write the store to disk
pub fn write_store_to_disk(store: &Store) {
    let store_str = format!("{:#?}", store);
    std::fs::write(".store.json", store_str).unwrap();
}
