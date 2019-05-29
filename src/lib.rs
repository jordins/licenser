use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
extern crate mktemp;
use mktemp::Temp;
use std::fs::OpenOptions;

pub fn prepend_content_to_file(
    content_to_prepend: &str,
    file_path_name: &str,
) -> Result<(), Box<dyn Error>> {
    let original_file_to_read = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path_name)
        .unwrap_or_else(|_| panic!("Could not open the file {}", file_path_name));

    // create temp file
    let temp_path =
        Temp::new_file().unwrap_or_else(|_| panic!("Could not create a temporary file"));

    let mut temp_file_to_write = OpenOptions::new()
        .write(true)
        .open(&temp_path)
        .unwrap_or_else(|_| panic!("Could not open a temporary file to write"));

    temp_file_to_write
        .write_all(content_to_prepend.as_bytes())
        .unwrap_or_else(|_| panic!("Could not write to the file {}", file_path_name));

    // read original file to buffer
    let mut buf_reader = BufReader::new(original_file_to_read);

    let mut content_from_original_file: Vec<u8> = Vec::new();
    buf_reader
        .read_to_end(&mut content_from_original_file)
        .expect("can't copy original file");

    // write to temp file
    temp_file_to_write
        .write_all(&content_from_original_file)
        .unwrap_or_else(|_| panic!("Could not write contents to temporary file"));

    // rename temp file to original file
    fs::rename(&temp_path, &file_path_name).unwrap_or_else(|_| panic!("Could not rename file"));
    temp_path.release();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prepends_properly_a_file() {
        let file_path_name = "./test/cleanfile2.txt";
        let original_file_contents = "1\n2\n3\nend of file!";
        create_test_file(&file_path_name, &original_file_contents);

        let content_to_prepend = "This is my prepended content \n";
        let mut expected_file_content = String::from(content_to_prepend).to_owned();
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
}
