use walkdir::{DirEntry, WalkDir};

pub fn list_files(in_dir: String) -> Vec<String> {
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

fn is_directory(dir_entry: &DirEntry) -> bool {
    return dir_entry.path().is_dir();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_files_should_not_list_directories() {
        let files: Vec<String> = list_files(String::from("./test/test-input-data"));
        assert_eq!(
            files,
            [
                "./test/test-input-data/subfolder/cleanfile.3.txt",
                "./test/test-input-data/cleanfile.2.txt",
                "./test/test-input-data/cleanfile.txt"
            ]
        );
    }
}
