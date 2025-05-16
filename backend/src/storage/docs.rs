use std::{collections::HashSet, fs, io};

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

pub fn save_doc(name: &str, content: &str, tags: &[String], editor: &str) -> io::Result<()> {
    let exists = load_doc(&name).is_ok();
    let result = if exists {
        edit_existing_doc(&name, &content, tags, editor)
    } else {
        create_new_doc(&name, content, tags, editor)
    };

    match result {
        Ok(_) => {
            tracing::info!("[{}] {}", name, if exists { "edited" } else { "created" });
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

fn create_new_doc(name: &str, content: &str, tags: &[String], editor: &str) -> io::Result<()> {
    let path = doc_path(name);
    if path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Document already exists"));
    }

    fs::write(&path, content)?;

    let meta = DocMeta::new(tags, editor);
    save_doc_meta(name, &meta)?;

    Ok(())
}

fn edit_existing_doc(name: &str, new_content: &str, tags: &[String], editor: &str) -> io::Result<()> {
    let old_content = load_doc(name).unwrap_or_default();
    let content_changed = old_content != new_content;

    let meta_result = load_doc_meta(name);

    let tag_changed = match &meta_result {
        Ok(meta) => &meta.tags != tags,
        Err(_) => true,
    };

    if !content_changed && !tag_changed {
        return Ok(());
    }

    if content_changed {
        fs::write(doc_path(name), new_content)?;
    }

    let mut meta = meta_result.unwrap_or_else(|_| DocMeta::new(tags, editor));
    if content_changed || tag_changed {
        meta.record_edit(editor, Some(&old_content), Some(new_content), tags);
    }

    save_doc_meta(name, &meta)
}

pub fn delete_doc_file(name: &str) -> std::io::Result<()> {
    fs::remove_file(doc_path(name))
}

pub fn find_docs_by_tag(tag: &str) -> io::Result<Vec<String>> {
    let mut matched = vec![];

    for name in list_doc_names()? {
        if let Ok(meta) = load_doc_meta(&name) {
            if meta.tags.contains(&tag.to_string()) {
                matched.push(name);
            }
        }
    }

    Ok(matched)
}

pub fn list_all_tags() -> io::Result<Vec<String>> {
    let names = list_doc_names().unwrap_or_default();
    let mut tag_set = HashSet::new();

    for name in names {
        if let Ok(meta) = load_doc_meta(&name) {
            for tag in meta.tags {
                tag_set.insert(tag);
            }
        }
    }

    let mut tags: Vec<_> = tag_set.into_iter().collect();
    tags.sort();
    Ok(tags)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{setup_test_env, clear_test_doc};

    use super::*;
    use std::fs;

    #[test]
    fn test_create_new_doc_success() {
        setup_test_env();

        let title = "create_new_doc_success";
        let content = "This is LiteWiki!";
        let editor = "test_editor";

        clear_test_doc(title);

        assert!(create_new_doc(title, content, &[], editor).is_ok());

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
        let editor = "test_editor";

        clear_test_doc(title);

        assert!(create_new_doc(title, original, &[], editor).is_ok());
        let result = create_new_doc(title, updated, &[], editor);
        assert!(matches!(result, Err(e) if e.kind() == io::ErrorKind::AlreadyExists));
    }

    #[test]
    fn test_edit_existing_doc_success() {
        setup_test_env();

        let title = "test_edit_doc";
        let original = "Original content";
        let updated = "Updated content";
        let editor = "test_editor";

        clear_test_doc(title);

        create_new_doc(title, original, &[], editor).unwrap();
        let result = edit_existing_doc(title,  updated, &[], "test_editor");
        assert!(result.is_ok());

        let saved = fs::read_to_string(doc_path(title)).unwrap();
        assert_eq!(saved, updated);

        let meta = load_doc_meta(title).unwrap();
        assert_eq!(meta.history.len(), 2);
        assert_eq!(meta.history.last().unwrap().summary, "save");
    }

    #[test]
    fn test_edit_existing_doc_no_diff() {
        setup_test_env();

        let title = "test_edit_existing_doc_no_diff";
        let content = "Hello world";
        let editor = "test_editor";

        clear_test_doc(title);

        create_new_doc(title, content, &[], editor).unwrap();
        assert!(edit_existing_doc(title, content, &[], "test_editor").is_ok());

        let meta = load_doc_meta(title).unwrap();
        assert_eq!(meta.history.len(), 1);
    }

    #[test]
    fn test_delete_doc_success() {
        setup_test_env();

        let title = "test_delete_doc";
        let content = "Content";
        let editor = "test_editor";

        clear_test_doc(title);
        
        create_new_doc(title, content, &[], editor).unwrap();
        
        assert!(delete_doc_file(title).is_ok());
        assert!(!doc_path(title).exists());
    }

    #[test]
    fn test_load_doc_success() {
        setup_test_env();

        let title = "test_load_doc";
        let content = "test load doc";
        let editor = "test_editor";

        clear_test_doc(title);

        create_new_doc(title, content, &[], editor).unwrap();

        let loaded = load_doc(title).unwrap();
        assert_eq!(content, loaded);
    }

    #[test]
    fn test_list_doc_names_returns_created_file() {
        setup_test_env();
        let title = "test_list_doc_names";
        let content = "checking file in list";
        let editor = "test_editor";

        clear_test_doc(title);
        create_new_doc(title, content, &[],  editor).unwrap();

        let names = list_doc_names().unwrap();
        assert!(names.contains(&title.to_string()));
    }
}