use std::{fs, path::PathBuf, io};
use crate::auth::user::User;
use crate::config::USER_DB_PATH;


fn load_users() -> io::Result<Vec<User>> {
    let data = fs::read_to_string(user_db_path()).unwrap();
    let users = serde_json::from_str(&data).unwrap();
    Ok(users)
}

fn save_users(users: &[User]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(users).unwrap();
    fs::write(user_db_path(), json)
}

fn user_db_path() -> PathBuf {
    USER_DB_PATH.clone().into()
}

pub fn get_user_by_name(username: &str) -> Option<User> {
    load_users().ok()?.into_iter().find(|u| u.username == username)
}

pub fn update_user<F>(username: &str, updater: F) -> io::Result<()>
where 
    F: FnOnce(&mut User),
{
    let mut users = load_users()?;
    if let Some(user) = users.iter_mut().find(|u| u.username == username) {
        updater(user);
        save_users(&users).unwrap();
    }
    Ok(())
}

pub fn add_user(new_user: User) -> io::Result<()> {
    let mut users = load_users()?;
    if users.iter().any(|u| u.username == new_user.username) {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "User already exists"))
    }

    users.push(new_user);
    save_users(&users)
}

pub fn list_all_users() -> io::Result<Vec<User>> {
    load_users()
}