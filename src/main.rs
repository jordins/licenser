use clap::{App, Arg};

fn main() {
    let matches = App::new("Licenser")
        .version("0.1.0")
        .author("Jordi Nistal <jordi.nistal@gmail.com>")
        .about("Adds license headers to your files")
        .arg(
            Arg::with_name("folder")
                .short("f")
                .long("folder")
                .value_name("FOLDER")
                .help("Specifies the folder where to add the licence"),
        )
        .arg(
            Arg::with_name("licensefile")
                .short("l")
                .long("licensefile")
                .value_name("LICENSE_FILE")
                .help("File with the license content"),
        )
        .get_matches();
    let folder = matches.value_of("folder").unwrap();
    println!("Value for folder: {}", folder);
    let licensefile = matches.value_of("licensefile").unwrap();
    println!("Value for licensefile: {}", licensefile);

    licenser::prepend("This is my new content\n", folder);
}
