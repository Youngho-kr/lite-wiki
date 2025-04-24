use std::{io, fs};
use crate::storage::*;

// 문서 본문 로드 및 저장
pub fn list_doc_names() -> io::Result<Vec<String>> {
    fs::create_dir_all(docs_dir_path())?;
    let entries = fs::read_dir(docs_dir_path())?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.path().file_name()?.to_str()?.to_string();
            if name.ends_with(".md") {
                Some(name.trim_end_matches(".md").to_string())
            } else {
                None
            }
        })
        .collect();
    Ok(entries)
}

pub fn load_doc(name: &str) -> io::Result<String> {
    let content = fs::read_to_string(doc_path(name))?;

    load_doc_meta(name).ok();

    Ok(content)
}

pub fn create_new_doc(name: &str, content: &str) -> io::Result<()> {
    let path = doc_path(name);
    if path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Document already exists"));
    }

    fs::write(&path, content)?;

    let meta = DocMeta::new("create");
    save_doc_meta(name, &meta)?;

    Ok(())
}

pub fn edit_existing_doc(name: &str, new_content: &str) -> io::Result<()> {
    let old_content = load_doc(name).unwrap_or_default();

    if old_content == new_content {
        return Ok(());
    }

    fs::write(doc_path(name), new_content)?;

    let mut meta = load_doc_meta(name).unwrap_or_default();
    meta.record_edit("save", Some(&old_content), Some(new_content));
    save_doc_meta(name, &meta)
}

pub fn delete_doc_file(name: &str) -> std::io::Result<()> {
    fs::remove_file(doc_path(name))
}



#[cfg(test)]
mod tests {
    use crate::test_utils::{setup_test_env, remove_test_doc};

    use super::*;
    use std::fs;

    #[test]
    fn test_create_new_doc_success() {
        setup_test_env();

        let title = "create_new_doc_success";
        let content = "This is LiteWiki!";

        remove_test_doc(title);

        assert!(create_new_doc(title, content).is_ok());

        let path = doc_path(title);
        assert!(path.exists());

        let saved = fs::read_to_string(path).unwrap();
        assert_eq!(saved, content);

        let meta = load_doc_meta(title).unwrap_or_default();
        assert_eq!(meta.history.last().unwrap().summary, "create");
    }

    #[test]
    fn test_create_new_doc_exists_fail() {
        setup_test_env();

        let title = "create_exists_doc_fail";
        let original = "This is LiteWiki!";
        let updated = "Is this LiteWiki?";

        remove_test_doc(title);

        assert!(create_new_doc(title, original).is_ok());
        let result = create_new_doc(title, updated);
        assert!(matches!(result, Err(e) if e.kind() == io::ErrorKind::AlreadyExists));
    }

    #[test]
    fn test_edit_existing_doc_success() {
        setup_test_env();

        let title = "test_edit_doc";
        let original = "Original content";
        let updated = "Updated content";

        remove_test_doc(title);

        create_new_doc(title, original).unwrap();
        let result = edit_existing_doc(title, updated);
        assert!(result.is_ok());

        let saved = fs::read_to_string(doc_path(title)).unwrap();
        assert_eq!(saved, updated);

        let meta = load_doc_meta(title).unwrap();
        assert_eq!(meta.history.len(), 2);
        assert_eq!(meta.history.last().unwrap().summary, "save");
    }

    #[test]
    fn test_delete_doc_success() {
        setup_test_env();

        let title = "test_delete_doc";
        let content = "Content";

        remove_test_doc(title);
        
        create_new_doc(title, content).unwrap();
        
        assert!(delete_doc_file(title).is_ok());
        assert!(!doc_path(title).exists());
    }

    #[test]
    fn test_load_doc_success() {
        setup_test_env();

        let title = "test_load_doc";
        let content = "test load doc";

        remove_test_doc(title);

        create_new_doc(title, content).unwrap();

        let loaded = load_doc(title).unwrap();
        assert_eq!(content, loaded);
    }

    #[test]
    fn test_list_doc_names_returns_created_file() {
        setup_test_env();
        let title = "test_list_doc_names";
        let content = "checking file in list";

        remove_test_doc(title);
        create_new_doc(title, content).unwrap();

        let names = list_doc_names().unwrap();
        assert!(names.contains(&title.to_string()));
    }
}