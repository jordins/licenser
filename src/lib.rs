use std::{fs, process};
pub mod filemanager;
pub mod prepender;

pub fn prepend(content: &str, folder: &str) {
    let files_to_modify = filemanager::list_files(folder.to_string());
    for file in files_to_modify {
        println!("Going to prepend content to {}", file);

        if let Err(e) = prepender::prepend_content_to_file(content, &file) {
            println!("Application error: {}", e);

            process::exit(1);
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
        let initial_files = filemanager::list_files(tmp_folder.to_string());
        for initial_file in initial_files {
            let expected_file = initial_file
                .replace(tmp_folder, expected_folder)
                .replace("initial", "");
            files_equal = filemanager::are_files_equal(&initial_file, &expected_file);
            if !files_equal {
                break;
            }
        }
        fs::remove_dir_all(tmp_folder).unwrap();
        assert_eq!(files_equal, true);
    }

}
