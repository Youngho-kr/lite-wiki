use once_cell::sync::Lazy;
use std::env;

pub static DATA_PATH: Lazy<String> = Lazy::new(|| {
    env::var("DATA_PATH").expect("Missing DATA_PATH in .env")
});

pub static TEMPLATE_PATH: Lazy<String> = Lazy::new(|| {
    env::var("TEMPLATE_PATH").expect("Missing TEMPLATE_PATH in .env")
});