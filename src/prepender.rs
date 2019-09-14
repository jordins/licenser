use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
extern crate mktemp;
use mktemp::Temp;
use std::fs::File;
use std::fs::OpenOptions;

pub fn prepend_content_to_file(
    content_to_prepend: &str,
    file_path_name: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path_name)?;
    let mut reader = BufReader::new(file);

    // create temp file
    let temp_path = Temp::new_file()?;
    let mut temp_file_to_write = OpenOptions::new().write(true).open(&temp_path)?;
    // prepare & write license to temp file
    let mut line = String::new();
    let mut is_first_line = true;
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        if is_first_line {
            is_first_line = false;
            line = get_shebang_and_license(&line, content_to_prepend);
        }
        temp_file_to_write
            .write_all(&line.as_bytes())
            .unwrap_or_else(|_| panic!("Could not write contents to temporary file"));
        line.clear();
    }

    // rename temp file to original file
    fs::rename(&temp_path, &file_path_name).unwrap_or_else(|_| panic!("Could not rename file"));
    temp_path.release();

    Ok(())
}

fn get_shebang_and_license(line: &str, license: &str) -> String {
    if has_shebangs(&line) {
        let mut shebang_and_licese = line.to_owned();
        shebang_and_licese.push_str("\n");
        shebang_and_licese.push_str(license);
        shebang_and_licese.push_str("\n");
        return shebang_and_licese.to_owned();
    } else {
        let mut license_and_firstline = license.to_owned();
        license_and_firstline.push_str("\n");
        license_and_firstline.push_str(&line);
        return license_and_firstline.to_owned();
    }
}

fn has_shebangs(content: &str) -> bool {
    let shebangs = vec![
        "#!",        // shell script
        "<?xml",     // XML declaration
        "<!doctype", // HTML doctype
    ];
    for shebang in shebangs {
        if content.contains(shebang) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prepends_properly_a_file() {
        let file_path_name = "./test/file2.txt";
        let original_file_contents = "1\n2\n3\nend of file!";
        create_test_file(&file_path_name, &original_file_contents);

        let content_to_prepend = "This is my prepended content";
        let mut expected_file_content = String::from(content_to_prepend).to_owned();
        expected_file_content.push_str("\n");
        expected_file_content.push_str(&original_file_contents);

        let result = prepend_content_to_file(content_to_prepend, &file_path_name);
        match result {
            Ok(_) => {
                let final_file_content = fs::read_to_string(file_path_name)
                    .expect("test failed because we can't read the concatenated file_path_name ");
                assert_eq!(final_file_content, expected_file_content);
            }
            Err(_) => assert_eq!(true, false),
        }
        fs::remove_file(file_path_name).unwrap();
    }

    fn create_test_file(file_path_name: &str, contents: &str) {
        fs::write(file_path_name, contents).unwrap();
    }

    #[test]
    fn has_shebangs_returns_true_if_text_contains_shebang() {
        assert!(has_shebangs("#!/bin/bash\necho hello"));
    }
    #[test]
    fn has_shebangs_returns_false_if_text_does_not_contain_shebang() {
        assert!(!has_shebangs("echo hello"));
    }
}
