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
        .arg(
            Arg::with_name("ignorefolders")
                .short("i")
                .long("ignorefolders")
                .value_name("IGNORE_FOLDERS")
                .multiple(true)
                .help("List of folders to ignore"),
        )
        .get_matches();
    let folder = matches.value_of("folder").unwrap();
    println!("Value for folder: {}", folder);
    let licensefile = matches.value_of("licensefile").unwrap();
    println!("Value for licensefile: {}", licensefile);
    let mut ignorefolders;
    match matches.values_of("ignorefolders") {
        None => ignorefolders = Vec::new(),
        Some(l) => ignorefolders = l.collect(),
    }

    licenser::prepend_from_license_file(licensefile, folder, &ignorefolders);
}
