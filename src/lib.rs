use std::{fs, process};
pub mod fileheader;
pub mod filemanager;
pub mod prepender;

pub fn prepend(license_content: &str, folder: &str) {
    let files_to_modify = filemanager::list_files(&folder);
    for file in files_to_modify {
        println!("Going to prepend content to {}", file);
        match filemanager::get_extension_from_file_path(file.as_str()) {
            Some(file_extension) => {
                let comented_file_header =
                    fileheader::FileHeader::new(license_content, file_extension)
                        .add_comments_to_content();
                if let Err(e) =
                    prepender::prepend_content_to_file(comented_file_header.content(), &file)
                {
                    println!("Application error: {}", e);
                    process::exit(1);
                }
            }
            None => println!(
                "Can't get extension for file {}, so won't add any licence to it",
                file
            ),
        }
    }
}

pub fn prepend_from_license_file(license_file: &str, folder: &str) {
    let license_content = fs::read_to_string(license_file).unwrap();
    prepend(&license_content, folder)
}

#[cfg(test)]
mod test {
    use super::*;
    use fs_extra::{dir::copy, dir::CopyOptions};

    #[test]
    fn prepends_properly_from_a_license_file() {
        let initial_folder = "./test/initial/";
        let expected_folder = "./test/expected";
        let tmp_folder = "./test/tmp";
        let mut copy_options = CopyOptions::new();
        copy_options.copy_inside = true;
        copy(initial_folder, tmp_folder, &copy_options).unwrap();
        let license_file_path = "./test/test-license-file.txt";
        prepend_from_license_file(license_file_path, &tmp_folder);

        let mut files_equal = true;
        let test_files = filemanager::list_files(&tmp_folder);
        for test_file in test_files {
            let expected_file = test_file
                .replace(tmp_folder, expected_folder)
                .replace("initial", "");
            files_equal = filemanager::are_files_equal(&test_file, &expected_file);
            if !files_equal {
                println!(
                    "This file is different: {} with content:\n{}",
                    &test_file,
                    fs::read_to_string(&test_file).unwrap()
                );
                break;
            }
        }
        fs::remove_dir_all(tmp_folder).unwrap();
        assert_eq!(files_equal, true);
    }

}
