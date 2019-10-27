//This is my license content
//This is my license content
//This is my license content
//! # Licenser
//!
//! Licenser is a tool to add a license header to all project files recursively.
//!
//!
//! ## Quick start
//! To add the contents of a license file into all your project files runthread:
//! ```
//! use licenser::*;
//! let license_file = "./test/test-license-file.txt";
//! let folder = "./licenser/src";
//! prepend_from_license_file(&license_file, &folder, &vec!["./src"]);
//! ```

use std::{fs, process};
mod fileheader;
mod filemanager;
mod prepender;

/// Prepends a `license` **string** recursively for each file inside the given `folder` ignoring the paths provided in `ignorefolders`
///
/// Each license will be prepended as a comment depending on the file type.
///
/// # Arguments
///
/// * `license_content` - A string slice that holds the license content to prepend
/// * `folder` - A string slice that holds the folder where your project is located
/// * `ignorefolders` - A reference to an array of string slices containing the paths you want to ignore.  
/// These won't include any license. You might want to add here folders with dependencies, assets or similar.
///
/// # Example
///
/// ```rust
/// use licenser::*;
/// let license_content = "This is my license content";
/// let folder = "./src";
/// prepend(&license_content, &folder, &vec!["./src"]);
/// ```
pub fn prepend(license_content: &str, folder: &str, ignorefolders: &[&str]) {
    let files_to_modify = filemanager::list_files(&folder, &ignorefolders);
    for file in files_to_modify {
        println!("Going to prepend license to {}", file);
        match filemanager::get_extension_from_file_path(file.as_str()) {
            Some(file_extension) => {
                let comented_file_header =
                    fileheader::FileHeader::new(license_content, file_extension)
                        .add_comments_to_license();
                if let Err(e) =
                    prepender::prepend_content_to_file(comented_file_header.license(), &file)
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

/// Prepends a `license` **file** recursively for each file inside the given `folder` ignoring the paths provided in `ignorefolders`
///
/// Each license will be prepended as a comment depending on the file type.
///
/// # Arguments
///
/// * `license_file` - A string slice that holds the path of the file with the license to prepend
/// * `folder` - A string slice that holds the folder where your project is located
/// * `ignorefolders` - A reference to an array of string slices containing the paths you want to ignore.  
/// These won't include any license. You might want to add here folders with dependencies, assets or similar.
///
/// # Example
///
/// ```rust
/// use licenser::*;
/// let license_file = "./test/test-license-file.txt";
/// let folder = "./licenser/src";
/// prepend_from_license_file(&license_file, &folder, &vec!["./src"]);
/// ```
pub fn prepend_from_license_file(license_file: &str, folder: &str, ignorefolders: &[&str]) {
    let license_content = fs::read_to_string(license_file).unwrap();
    prepend(&license_content, folder, ignorefolders)
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
        let ignoredfolders = &vec!["./test/tmp/ignoreme"];
        let mut copy_options = CopyOptions::new();
        copy_options.copy_inside = true;
        copy(initial_folder, tmp_folder, &copy_options).unwrap();
        let license_file_path = "./test/test-license-file.txt";
        prepend_from_license_file(license_file_path, &tmp_folder, ignoredfolders);

        let mut files_equal = true;
        let test_files = filemanager::list_files(&tmp_folder, ignoredfolders);
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
