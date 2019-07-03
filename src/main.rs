extern crate copyrightfiles;
use std::process;

fn main() {
    let files_to_modify =
        copyrightfiles::filemanager::list_files("./test/test-input-data".to_string());
    for file in files_to_modify {
        let content = "This is my new content\n";
        println!("Going to prepend content to {}", file);

        if let Err(e) = copyrightfiles::prepender::prepend_content_to_file(content, &file) {
            println!("Application error: {}", e);

            process::exit(1);
        }
    }
}
