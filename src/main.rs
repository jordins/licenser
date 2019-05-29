extern crate copyrightfiles;
use std::process;

fn main() {
    let file_path_name = "./test/cleanfile.txt";
    let content = "This is my new content\n";

    if let Err(e) = copyrightfiles::prepend_content_to_file(content, &file_path_name) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
