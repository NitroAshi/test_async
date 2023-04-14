use std::path::{Path, PathBuf};
use std::fs;
use clap::{App, Arg};

fn main() {
    let matches = App::new("file_finder")
        .arg(Arg::with_name("path")
            .required(true)
            .multiple(true)
            .takes_value(true)
            .help("path to the file or directory"))
        .get_matches();

    let mut file_paths: Vec<PathBuf> = Vec::new();

    for path_str in matches.values_of("path").unwrap() {
        let path = Path::new(path_str);

        if path.is_file() && path.extension().unwrap() == "txt" {
            file_paths.push(path.to_owned());
        } else if path.is_dir() {
            let txt_files = fs::read_dir(path)
                .expect("Failed to read directory")
                .filter_map(Result::ok)
                .filter(|f| f.path().extension().unwrap_or_default() == "txt")
                .map(|f| f.path())
                .collect::<Vec<PathBuf>>();

            file_paths.extend(txt_files);
        }
    }

    println!("{:?}", file_paths);
}

