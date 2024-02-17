use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Revision {
    id: u64,
    timestamp: String,
    minor: bool,
    size: u32,
    comment: String,
    user: User,
    delta: i32,
}

// Struct for the response from the API
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct RevisionResponse {
    revisions: Vec<Revision>,
    newer: Option<String>,
    older: Option<String>,
    latest: Option<String>,
}

// Struct for the user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
}
