pub struct FileHeader {
    content: String,
    fileextension: String,
}

impl FileHeader {
    pub fn new(content: &str, fileextension: &str) -> FileHeader {
        FileHeader {
            content: String::from(content),
            fileextension: String::from(fileextension),
        }
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn add_comments_to_content(&self) -> FileHeader {
        let comment = get_comment_from_extension(self.fileextension.as_str());
        let commented_license = self.add_comment_symbol_to_each_line_of_license(comment);
        return FileHeader {
            content: commented_license,
            fileextension: String::from(&self.fileextension),
        };
    }

    fn is_multiline_content(&self) -> bool {
        return self.content.contains("\n");
    }

    fn add_comment_symbol_to_each_line_of_license(&self, comment: Comment) -> String {
        let mut commented_license: String;

        commented_license = if self.is_multiline_content()
            && !comment.start.is_empty()
            && !comment.end.is_empty()
        {
            comment.start + "\n" + &self.content + &comment.end + "\n"
        } else {
            self.content
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
    fn add_comments_to_content_returns_a_commented_content_in_js() {
        let file_header = FileHeader {
            content: String::from("my content"),
            fileextension: String::from("js"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_content();
        assert_eq!(new_struct.content, "//my content");
    }

    #[test]
    fn add_comments_to_content_returns_same_content_if_not_recognised_extension() {
        let file_header = FileHeader {
            content: String::from("my content"),
            fileextension: String::from("unknown_extension"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_content();
        assert_eq!(new_struct.content, "my content");
    }

    #[test]
    fn add_comments_to_multiline_content_in_js() {
        let file_header = FileHeader {
            content: String::from("my content\nmy new line\nAnother line"),
            fileextension: String::from("js"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_content();
        assert_eq!(
            new_struct.content,
            "/*\nmy content\nmy new line\nAnother line*/\n"
        );
    }

    #[test]
    fn add_comments_to_multiline_content_in_bash() {
        let file_header = FileHeader {
            content: String::from("my content\nmy new line\nAnother line"),
            fileextension: String::from("sh"),
        };
        let new_struct: FileHeader = file_header.add_comments_to_content();
        assert_eq!(
            new_struct.content,
            "# my content\n# my new line\n# Another line"
        );
    }

}
