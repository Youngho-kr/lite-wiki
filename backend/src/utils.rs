use std::path::Path;
use crate::config::{DATA_PATH, TEMPLATE_PATH, USER_DB_PATH};

pub fn check_environment_directories() {
    if !Path::new(&*DATA_PATH).is_dir() {
        panic!("DATA_PATH does not exist: {:?}", *DATA_PATH);
    }
    if !Path::new(&*TEMPLATE_PATH).is_dir() {
        panic!("TEMPLATE_PATH does not exist: {:?}", *TEMPLATE_PATH);
    }
    if !Path::new(&*USER_DB_PATH).exists() {
        panic!("USER_DB_PATH does not exist: {:?}", *USER_DB_PATH);
    }
}