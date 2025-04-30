use std::{fs, path::PathBuf, io};
use crate::auth::user::User;
use crate::config::USER_DB_PATH;


pub fn load_users() -> io::Result<Vec<User>> {
    let data = fs::read_to_string(user_db_path()).unwrap();
    let users = serde_json::from_str(&data).unwrap();
    Ok(users)
}

pub fn save_users(users: &[User]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(users).unwrap();
    fs::write(user_db_path(), json)
}

fn user_db_path() -> PathBuf {
    USER_DB_PATH.clone().into()
}