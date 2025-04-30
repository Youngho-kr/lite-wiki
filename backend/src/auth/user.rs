use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub is_authorized: bool,
}
