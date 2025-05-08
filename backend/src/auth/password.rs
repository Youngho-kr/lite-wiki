use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use serde::Deserialize;

use super::{get_user_by_name, update_user, User};

#[derive(Deserialize)]
pub struct ChangePasswordReqeust {
    pub current: String,
    pub new: String,
}

pub fn change_password(user: User, password: &str) -> Result<(), std::io::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    update_user(&user.username, |u| { u.password_hash = hashed.clone(); })
}

pub fn verify_password(username: &str, password: &str) -> Option<User> {
    let user = match get_user_by_name(username) {
        Some(u) => u,
        None => return None,
    };

    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(_) => return None,
    };

    match Argon2::default().verify_password(&password.as_bytes(), &parsed_hash) {
        Ok(_) => Some(user),
        Err(_) => None,
    }
}