use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Revision {
    pub id: u64,
    pub timestamp: String,
    pub minor: bool,
    pub size: u32,
    pub comment: String,
    pub user: User,
    pub delta: i32,
}

// Struct for the response from the API
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct RevisionResponse {
    pub revisions: Vec<Revision>,
    pub newer: Option<String>,
    pub older: Option<String>,
    pub latest: Option<String>,
}

// Struct for the user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}
