use serde::{Deserialize, Serialize};

// Struct for the revision count
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct CountResponse {
    pub count: u64,
    pub limit: bool,
}

// Struct for table of revisions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct RevisionTable {
    pub page: String,
    pub revisions: Revision,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct Revision {
    pub id: u64,
    pub timestamp: Option<String>,
    pub minor: Option<bool>,
    pub size: Option<u32>,
    pub comment: Option<String>,
    pub user: Option<User>,
    pub delta: Option<i32>,
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
    pub id: Option<u64>,
    pub name: Option<String>,
}
