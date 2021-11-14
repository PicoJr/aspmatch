use aspmatch::parse_binary_match_file_path;
use clap::{crate_authors, crate_version, App, Arg};
use std::path::PathBuf;

extern crate clap;

fn main() {
    let matches = App::new("info")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Print match file content")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("match file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let input_file = matches.value_of("input").expect("INPUT is required");
    match parse_binary_match_file_path(PathBuf::from(input_file)) {
        Ok(ipmatch) => {
            println!("{}", ipmatch.as_text());
        }
        Err(e) => {
            eprintln!("error: {:?}", e);
        }
    }
}
