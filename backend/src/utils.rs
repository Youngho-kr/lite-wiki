use std::path::Path;
use crate::config::{DOCS_PATH, SETTINGS_PATH, UPLOADS_PATH, USER_DB_PATH};

pub fn check_environment_directories() {
    if !Path::new(&*DOCS_PATH).is_dir() {
        panic!("DATA_PATH does not exist: {:?}", *DOCS_PATH);
    }
    if !Path::new(&*UPLOADS_PATH).is_dir() {
        panic!("UPLOADS_PATH does not exist: {:?}", *UPLOADS_PATH);
    }
    if !Path::new(&*USER_DB_PATH).exists() {
        panic!("USER_DB_PATH does not exist: {:?}", *USER_DB_PATH);
    }
    if !Path::new(&*SETTINGS_PATH).exists() {
        panic!("SETTINGS_PATH does not exist: {:?}", *SETTINGS_PATH);
    }
}