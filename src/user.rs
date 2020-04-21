use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub name: String,
    pub age: u32,
}
