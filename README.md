# Licenser

Licenser is a tool to add a license header to all project files recursively.

You can run it like this:
```
cargo run -- -f /path/to/my/project/folder -l /path/to/my/licensefile -i /path/to/my/project/ignoredfolder1 /path/to/my/project/ignoredfolder2
````

This will add the contents of `licensefile` to the beginning of each file
adding it as a comment.

The comment symbols are generated depending on the file type. For instance, for a javascript file (`.js`) it will add the license wrapped between `/*` and `*/`.
It supports several languages and file extensions like: `go`, `bash`, `rust`, `java`, `html` and more.

It also takes care of shebangs ;)

### How to ignore folders or files
By passing the `-i` or `--ignorefolders` flag with a list of paths `licenser` won't add a license to them.
At the moment the paths should not end with a `/` symbol.

To see available commands run:
```
cargo run -- -h
````

For tests run:
```
cargo test
````

## Collaboration
If you need any of file extension that we don't have, just submit an issue and we will add it or just contribute with a PR!
