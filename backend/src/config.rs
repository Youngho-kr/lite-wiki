use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, fs, sync::RwLock};

pub static DATA_PATH: Lazy<String> = Lazy::new(|| env_required("DATA_PATH"));
pub static TEMPLATE_PATH: Lazy<String> = Lazy::new(|| env_required("TEMPLATE_PATH"));
pub static USER_DB_PATH: Lazy<String> = Lazy::new(|| env_required("USER_DB_PATH"));
pub static SETTINGS_PATH: Lazy<String> = Lazy::new(|| env_required("SETTINGS_PATH"));
pub static JWT_SECRET: Lazy<String> = Lazy::new(|| env_required("JWT_SECRET_KEY"));

fn env_required(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Mssing {} in .env", key))
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub redirect_page: String,
}

pub static REDIRECT_PAGE: Lazy<RwLock<String>> = Lazy::new(|| {
    let settings_json = fs::read_to_string(SETTINGS_PATH.clone()).expect("Failed to read settings file");
    let settings: Settings = serde_json::from_str(&settings_json).expect("invalid settings.json format");
    RwLock::new(settings.redirect_page)
});

pub fn get_redirect_page() -> String {
    REDIRECT_PAGE.read().unwrap().clone()
}

pub fn save_redirect_setting(redirect: &str) -> std::io::Result<()> {
    let redirect_to = if redirect.starts_with('/') {
        redirect
    } else {
        &format!("/{}", redirect)
    };

    let settings = Settings {
        redirect_page: redirect_to.to_string(),
    };

    let json = serde_json::to_string_pretty(&settings)?;
    fs::write(SETTINGS_PATH.as_str(), json)?;

    *REDIRECT_PAGE.write().unwrap() = redirect_to.to_string();
    Ok(())
}
