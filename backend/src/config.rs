use once_cell::sync::Lazy;
use std::env;

pub static DATA_PATH: Lazy<String> = Lazy::new(|| {
    env::var("DATA_PATH").expect("Missing DATA_PATH in .env")
});

pub static TEMPLATE_PATH: Lazy<String> = Lazy::new(|| {
    env::var("TEMPLATE_PATH").expect("Missing TEMPLATE_PATH in .env")
});

pub static USER_DB_PATH: Lazy<String> = Lazy::new(|| {
    env::var("USER_DB_PATH").expect("Mssing USER_DB_PATH in .env")
});

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET_KEY").expect("Missing JWT_SECRET_KEY in .env")
});