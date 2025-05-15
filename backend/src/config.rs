use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, fs, sync::RwLock};

// Required environment variables (panic on missing)
pub static DOCS_PATH: Lazy<String> = Lazy::new(|| env_required("DOCS_PATH"));
pub static UPLOADS_PATH: Lazy<String> = Lazy::new(|| env_required("UPLOADS_PATH"));
pub static USER_DB_PATH: Lazy<String> = Lazy::new(|| env_required("USER_DB_PATH"));
pub static SETTINGS_PATH: Lazy<String> = Lazy::new(|| env_required("SETTINGS_PATH"));
pub static JWT_SECRET: Lazy<String> = Lazy::new(|| env_required("JWT_SECRET_KEY"));
pub static BASE_URL: Lazy<String> = Lazy::new(|| env_required("BASE_URL"));

fn env_required(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Mssing required environment variable: {}", key))
}

// Settings structure for persistent config
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub redirect_page: String,
}

// In-memory cached redirect page (loaded from settings file)
pub static REDIRECT_PAGE: Lazy<RwLock<String>> = Lazy::new(|| {
    let settings_json = fs::read_to_string(&*SETTINGS_PATH).expect("Failed to read settings.json file");
    let settings: Settings = serde_json::from_str(&settings_json).expect("Invalid settings.json format");
    RwLock::new(settings.redirect_page)
});

// Get the currently configured redirect page (read from in-memory cache)
pub fn current_redirect_page() -> String {
    REDIRECT_PAGE.read().unwrap().clone()
}

// Update redirect page to both memory and file
pub fn update_redirect_page(redirect: &str) -> std::io::Result<()> {
    let redirect_to = if redirect.starts_with('/') {
        redirect.to_string()
    } else {
        format!("/{}", redirect)
    };

    let setting_json = fs::read_to_string(&*SETTINGS_PATH)?;
    let mut settings: Settings = serde_json::from_str(&setting_json)?;

    settings.redirect_page = redirect_to.clone();
    
    let json = serde_json::to_string_pretty(&settings)?;
    fs::write(SETTINGS_PATH.as_str(), json)?;

    *REDIRECT_PAGE.write().unwrap() = redirect_to.to_string();
    Ok(())
}
