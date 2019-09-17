# Licenser

Licenser is a tool to add a license header to all project files recursively.

You can run it like this:
```
cargo run -- -f /path/to/my/project/folder -l /path/to/my/licensefile
````

This will add the contents of `licensefile` to the beginning of each file
adding it as a comment.

It also takes care of shebangs ;)

For help run:
```
cargo run -- -h
````

For tests run:
```
cargo test
````
