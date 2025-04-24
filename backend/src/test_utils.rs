use std::env;
use std::fs;
use std::path::Path;


pub fn setup_test_env() {
    // .env.test 로드
    dotenvy::from_filename(".env.test").expect("Failed to load .env.test");

    // 테스트용 데이터 디렉토리 생성
    let test_data_dir = env::var("DATA_PATH").expect("DATA_PATH must be set in .env.test");
    if !Path::new(&test_data_dir).exists() {
        fs::create_dir_all(&test_data_dir).expect("Failed to create test data directory");
    }

    let test_template_dir = env::var("TEMPLATE_PATH").expect("TEMPLATE_PATH must be set in .env.test");
    if !Path::new(&test_template_dir).exists() {
        fs::create_dir_all(&test_template_dir).expect("Failed to create test template directory");
    }
}

/// 테스트용 디렉토리 정리 (선택사항)
pub fn clear_test_docs() {
    let test_data_dir = env::var("DATA_PATH").unwrap();
    let _ = fs::remove_dir_all(&test_data_dir);
    let _ = fs::create_dir_all(&test_data_dir);
}