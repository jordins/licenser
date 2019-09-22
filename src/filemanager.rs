use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn list_files(in_dir: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(in_dir) {
        match entry {
            Ok(dir_entry) => {
                if !is_directory(&dir_entry) {
                    let files_list = dir_entry.path().to_str().unwrap();
                    files.push(String::from(files_list));
                }
            }
            _ => println!("Error reading file"),
        };
    }
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

fn is_directory(dir_entry: &DirEntry) -> bool {
    dir_entry.path().is_dir()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_files_should_not_list_directories() {
        let mut files: Vec<String> = list_files("./test/initial");
        files.sort();
        let mut expected = [
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
