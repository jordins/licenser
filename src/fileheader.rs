pub struct FileHeader {
    license: String,
    fileextension: String,
}

impl FileHeader {
    pub fn new(license: &str, fileextension: &str) -> FileHeader {
        FileHeader {
            license: String::from(license),
            fileextension: String::from(fileextension),
        }
    }

    pub fn license(&self) -> &String {
        &self.license
    }

    pub fn add_comments_to_license(&self) -> FileHeader {
        let comment = get_comment_from_extension(self.fileextension.as_str());
        let commented_license = self.add_comment_symbol_to_each_line_of_license(comment);
        return FileHeader {
            license: commented_license,
            fileextension: String::from(&self.fileextension),
        };
    }

    fn is_multiline_license(&self) -> bool {
        return self.license.contains("\n");
    }

    fn add_comment_symbol_to_each_line_of_license(&self, comment: Comment) -> String {
        let mut commented_license: String;

        commented_license = if self.is_multiline_license()
            && !comment.start.is_empty()
            && !comment.end.is_empty()
        {
            comment.start + "\n" + &self.license + &comment.end + "\n"
        } else {
            self.license
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    if i != 0 {
                        return String::from("\n") + &comment.line + line;
                    }
                    return String::from(&comment.line) + line;
                })
                .collect()
        };
        commented_license
    }
}

struct Comment {
    start: String,
    line: String,
    end: String,
}

fn get_comment_from_extension(extension: &str) -> Comment {
    match extension {
        "js" | "jsx" | "ts" | "tsx" | "java" | "go" | "rs" | "css" | "tf" => Comment {
            start: String::from("/*"),
            end: String::from("*/"),
            line: String::from("//"),
        },
        "py" | "sh" | "yaml" | "yml" | "dockerfile" | "rb" | "gemfile" => Comment {
            start: String::from(""),
            end: String::from(""),
            line: String::from("# "),
        },
        "html" | "xml" => Comment {
            start: String::from("<!--"),
            end: String::from("-->"),
            line: String::from(""),
        },
        _ => Comment {
            start: String::from(""),
            end: String::from(""),
            line: String::from(""),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_comments_to_license_returns_a_commented_license_in_js() {
        let file_header = FileHeader {
            license: String::from("my license"),
            fileextension: String::from("js"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_license();
        assert_eq!(new_struct.license, "//my license");
    }

    #[test]
    fn add_comments_to_license_returns_same_license_if_not_recognised_extension() {
        let file_header = FileHeader {
            license: String::from("my license"),
            fileextension: String::from("unknown_extension"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_license();
        assert_eq!(new_struct.license, "my license");
    }

    #[test]
    fn add_comments_to_multiline_license_in_js() {
        let file_header = FileHeader {
            license: String::from("my license\nmy new line\nAnother line"),
            fileextension: String::from("js"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_license();
        assert_eq!(
            new_struct.license,
            "/*\nmy license\nmy new line\nAnother line*/\n"
        );
    }

    #[test]
    fn add_comments_to_multiline_license_in_bash() {
        let file_header = FileHeader {
            license: String::from("my license\nmy new line\nAnother line"),
            fileextension: String::from("sh"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_license();
        assert_eq!(
            new_struct.license,
            "# my license\n# my new line\n# Another line"
        );
    }

}
