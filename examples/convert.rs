extern crate clap;

use aspmatch::{
    dump_match_as_binary, dump_match_as_binary_to_path, dump_match_as_text,
    dump_match_as_text_to_path, parse_binary_match_file_path, parse_text_match_file_path,
    ASPMatchError, IPMatch,
};
use clap::{crate_authors, crate_version, App, Arg};
use std::io;
use std::path::{Path, PathBuf};

enum Output {
    Stdout,
    FilePath(PathBuf),
}

fn parse_match_file_path<P: AsRef<Path>>(
    path: P,
    match_file_is_binary: bool,
) -> Result<IPMatch, ASPMatchError> {
    if match_file_is_binary {
        parse_binary_match_file_path(path)
    } else {
        parse_text_match_file_path(path)
    }
}

fn main() -> Result<(), ASPMatchError> {
    let matches = App::new("convert")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(
            "Convert match file binary -> text (or text -> binary if `-rev/--reverse` is provided)",
        )
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("match file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT")
                .help("match file (if omitted print to stdout instead)")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rev")
                .short("rev")
                .long("reverse")
                .help("reverse conversion: text -> binary")
                .required(false)
                .takes_value(false),
        )
        .get_matches();
    let input_file = matches.value_of("input").expect("input is required");
    let output_file = matches.value_of("output");
    let reverse = matches.is_present("rev");

    let output = match output_file {
        None => Output::Stdout,
        Some(output_path) => Output::FilePath(PathBuf::from(output_path)),
    };

    match parse_match_file_path(PathBuf::from(input_file), !reverse) {
        Ok(ipmatch) => {
            match output {
                Output::Stdout => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();
                    if reverse {
                        // text -> binary
                        dump_match_as_binary(&ipmatch, &mut handle)?;
                    } else {
                        // binary -> text
                        dump_match_as_text(&ipmatch, &mut handle)?;
                    }
                }
                Output::FilePath(path) => {
                    if reverse {
                        // text -> binary
                        dump_match_as_binary_to_path(&ipmatch, path)?;
                    } else {
                        // binary -> text
                        dump_match_as_text_to_path(&ipmatch, path)?;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("error: {:?}", e);
        }
    }
    Ok(())
}
