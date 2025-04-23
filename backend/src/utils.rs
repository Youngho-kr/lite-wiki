use std::path::Path;
use crate::config::{DATA_PATH, TEMPLATE_PATH};

pub fn check_environment_directories() {
    if !Path::new(&*DATA_PATH).is_dir() {
        panic!("DATA_PATH does not exist: {:?}", *DATA_PATH);
    }
    if !Path::new(&*TEMPLATE_PATH).is_dir() {
        panic!("TEMPLATE_PATH does not exist: {:?}", *TEMPLATE_PATH);
    }
}