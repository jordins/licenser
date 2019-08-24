use std::process;
pub mod filemanager;
pub mod prepender;

pub fn append() {
    let files_to_modify = filemanager::list_files("./test/test-input-data".to_string());
    for file in files_to_modify {
        let content = "This is my new content\n";
        println!("Going to prepend content to {}", file);

        if let Err(e) = prepender::prepend_content_to_file(content, &file) {
            println!("Application error: {}", e);

            process::exit(1);
        }
    }
}
