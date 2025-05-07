use std::{fs, path::PathBuf, io};
use crate::config::TEMPLATE_PATH;

pub fn list_template_names() -> Vec<String> {
    let mut names = vec![];
    if let Ok(entries) = fs::read_dir(TEMPLATE_PATH.clone()) {
        for entry in entries.flatten() {
            if let Some(name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                names.push(name.to_string());
            }
        }
    }
    names
}

pub fn load_template(name: &str) -> io::Result<String> {
    let mut path = PathBuf::from(TEMPLATE_PATH.clone());
    path.push(format!("{name}.md"));
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};
    use crate::{config::TEMPLATE_PATH, storage::{list_template_names, load_template}, test_utils::{clear_test_template, setup_test_env}};

    fn write_template(title: &str, content: &str) {
        let dir = TEMPLATE_PATH.clone();
        let path = format!("{}/{}.md", dir, title);
        fs::write(PathBuf::from(path), content).unwrap();
    }

    #[test]
    fn test_list_template_names() {
        setup_test_env();

        let title = "test_list_template_names";
        let content = "template_content";

        clear_test_template(title);

        write_template(title, content);

        assert!(list_template_names().contains(&title.to_string()));
    }

    #[test]
    fn test_load_template() {
        setup_test_env();

        let title = "test_load_template";
        let content = "template content";

        clear_test_template(title);

        write_template(title, content);

        let loaded = load_template(title).unwrap();

        assert_eq!(loaded, content);
    }
}