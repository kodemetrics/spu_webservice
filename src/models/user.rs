use serde::{Deserialize, Serialize};

#[derive(Clone,Debug, serde::Serialize,serde::Deserialize)]
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>
}