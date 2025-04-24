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
    use crate::test_utils::{clear_test_docs, setup_test_env};

    use super::*;
    use std::fs;

    fn cleanup(name: &str) {
        let _ = delete_doc_file(name);
        let _ = fs::remove_file(crate::storage::path::doc_meta_path(name));
    }

    #[test]
    fn test_create_edit_load_delete_doc() {
        setup_test_env();
        clear_test_docs();

        let name = "test_module_doc";
        let content1 = "Hello, world!";
        let content2 = "Updated content!";

        cleanup(name);
        
        // create
        create_new_doc(name, content1).expect("create failed");
        
        // load
        let loaded = load_doc(name).expect("load failed");
        assert_eq!(loaded, content1);

        // edit
        edit_existing_doc(name, content2).expect("edit failed");
        let edited = load_doc(name).expect("load after edit failed");
        assert_eq!(edited, content2);

        // list
        let list = list_doc_names().expect("list failed");
        assert!(list.contains(&name.to_string()));

        // delete
        delete_doc_file(name).expect("delete failed");
        assert!(!doc_path(name).exists());

        cleanup(name);
    }
}