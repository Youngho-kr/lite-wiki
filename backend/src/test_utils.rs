use std::env;
use std::fs;
use std::path::Path;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup_test_env() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").expect("Failed to load .env.test");
    });

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

pub fn clear_test_docs() {
    let test_data_dir = env::var("DATA_PATH").unwrap();

    let _ = fs::remove_dir_all(&test_data_dir);
    let _ = fs::create_dir_all(&test_data_dir);
}

use std::path::PathBuf;

pub fn clear_test_doc(title: &str) {
    // 환경 변수에서 DATA_PATH 가져오기
    let base = env::var("DATA_PATH").expect("Missing DATA_PATH");

    // 파일 경로 문자열 직접 조합
    let doc_path = PathBuf::from(format!("{}/{}.md", base, title));
    let meta_path = PathBuf::from(format!("{}/{}.meta.json", base, title));

    // 파일 삭제 시도
    let _ = fs::remove_file(doc_path);
    let _ = fs::remove_file(meta_path);
}