use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn list_files(in_dir: &str, ignored_dirs: &[&str]) -> Vec<String> {
    let cleaned_ignored_dirs: Vec<String> = remove_last_slash_from_path(&Vec::from(ignored_dirs));
    let mut files: Vec<String> = Vec::new();
    WalkDir::new(in_dir)
        .into_iter()
        .filter_entry(|e| {
            !is_ignored(
                e,
                &cleaned_ignored_dirs
                    .as_slice()
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
            )
        })
        .filter(|e| is_valid_file(e))
        .for_each(|entry| files.push(get_filename_from_direntry(entry)));
    files
}

pub fn are_files_equal(file_path1: &str, file_path2: &str) -> bool {
    let contents1 = fs::read_to_string(file_path1)
        .unwrap_or_else(|_| panic!("Could not read file {}", file_path1));
    let contents2 = fs::read_to_string(file_path2)
        .unwrap_or_else(|_| panic!("Could not read file {}", file_path2));
    contents1 == contents2
}

pub fn get_extension_from_file_path(file_path: &str) -> Option<&str> {
    Path::new(file_path).extension().and_then(OsStr::to_str)
}

fn remove_last_slash_from_path<'a>(paths: &[&'a str]) -> Vec<String> {
    paths
        .iter()
        .map(|dir| {
            let dir_owned = String::from(*dir);
            if dir_owned.ends_with('/') {
                return dir_owned.chars().take(dir_owned.len() - 1).collect();
            }
            dir_owned
        })
        .collect()
}

fn is_valid_file(entry: &Result<DirEntry, walkdir::Error>) -> bool {
    let mut is_valid = false;
    match entry {
        Ok(dir_entry) => {
            if !is_directory(&dir_entry) {
                is_valid = true
            }
        }
        _ => is_valid = false,
    }
    is_valid
}

fn get_filename_from_direntry(entry: Result<DirEntry, walkdir::Error>) -> String {
    let dir_entry = entry.unwrap();
    let files_list = dir_entry.path().to_str().unwrap();
    String::from(files_list)
}

fn is_ignored(e: &DirEntry, ignored_dirs: &[&str]) -> bool {
    ignored_dirs.contains(&e.path().to_str().unwrap())
}

fn is_directory(dir_entry: &DirEntry) -> bool {
    dir_entry.path().is_dir()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_files_should_not_list_directories() {
        let files: Vec<String> = list_files("./test/initial", &Vec::new());
        let expected = vec![
            "./test/initial/subfolder/file.3.txt",
            "./test/initial/file.rs",
            "./test/initial/file.txt",
            "./test/initial/file.go",
            "./test/initial/file.2.txt",
            "./test/initial/file.sh",
            "./test/initial/no-extension-file",
            "./test/initial/file.js",
            "./test/initial/file.html",
            "./test/initial/ignoreme/ignoreme.txt",
        ];
        assert_files_equals_expected(files, expected);
    }

    #[test]
    fn list_files_should_not_list_files_inside_ignored_directories() {
        let files: Vec<String> = list_files("./test/initial", &vec!["./test/initial/ignoreme"]);
        let expected = vec![
            "./test/initial/subfolder/file.3.txt",
            "./test/initial/file.rs",
            "./test/initial/file.txt",
            "./test/initial/file.go",
            "./test/initial/file.2.txt",
            "./test/initial/file.sh",
            "./test/initial/no-extension-file",
            "./test/initial/file.js",
            "./test/initial/file.html",
        ];
        assert_files_equals_expected(files, expected);
    }

    #[test]
    fn list_files_should_not_list_files_inside_ignored_directories_when_paths_ends_with_slash() {
        let files: Vec<String> = list_files("./test/initial", &vec!["./test/initial/ignoreme/"]);
        let expected = vec![
            "./test/initial/subfolder/file.3.txt",
            "./test/initial/file.rs",
            "./test/initial/file.txt",
            "./test/initial/file.go",
            "./test/initial/file.2.txt",
            "./test/initial/file.sh",
            "./test/initial/no-extension-file",
            "./test/initial/file.js",
            "./test/initial/file.html",
        ];
        assert_files_equals_expected(files, expected);
    }

    fn assert_files_equals_expected(mut files: Vec<String>, mut expected: Vec<&str>) {
        files.sort();
        expected.sort();
        assert_eq!(files, expected);
    }

    #[test]
    fn are_files_equal_should_return_true_when_equal() {
        let file_path1 = "test/initial/file.txt";
        let file_path2 = "test/initial/subfolder/file.3.txt";
        let equal = are_files_equal(file_path1, file_path2);
        assert_eq!(equal, true);
    }

    #[test]
    fn are_files_equal_should_return_false_when_different_content() {
        let file_path1 = "test/initial/file.txt";
        let file_path2 = "test/test-license-file.txt";
        let equal = are_files_equal(file_path1, file_path2);
        assert_eq!(equal, false);
    }

    #[test]
    fn get_extension_from_file_path_should_return_the_extension() {
        assert_eq!(
            get_extension_from_file_path("test/test-license-file.txt"),
            Some("txt")
        );
    }
}
