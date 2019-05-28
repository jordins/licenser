use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
extern crate mktemp;
use mktemp::Temp;
use std::fs::OpenOptions;

fn main() {

    let file_path_name = "./test/cleanfile.txt";
    let content = "This is my new content\n";

    prepend_content_to_file(content, &file_path_name);

}

fn prepend_content_to_file(content_to_prepend: &str, file_path_name: &str) {
    let original_file_to_read = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path_name)
            .unwrap_or_else(|_| panic!("Could not open the file {}", file_path_name));

    // create temp file
    let temp_path =
        Temp::new_file().unwrap_or_else(|_| panic!("Could not create a temporary file"));;
    

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
    fs::rename(&temp_path, &file_path_name)
        .unwrap_or_else(|_| panic!("Could not rename file"));
    temp_path.release();
}
