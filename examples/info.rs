use aspmatch::parse_match_file_path;
use clap::{crate_authors, crate_version, App, Arg};
use std::path::PathBuf;

extern crate clap;

fn main() {
    let matches = App::new("My Super Program")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Does awesome things")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("match file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let input_file = matches.value_of("input").expect("INPUT is required");
    match parse_match_file_path(PathBuf::from(input_file)) {
        Ok(ipmatch) => {
            println!("{:?}", ipmatch);
        }
        Err(e) => {
            eprintln!("error: {:?}", e);
        }
    }
}
