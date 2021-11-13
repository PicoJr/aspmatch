use aspmatch::{parse_match_file_path, IPMatch, IPRecord};
use clap::{crate_authors, crate_version, App, Arg};
use std::path::PathBuf;

extern crate clap;

fn iprecord_as_text(iprecord: &IPRecord) -> String {
    format!(
        "{} {} {} {} {} {} {} {} {} {} {} {}",
        iprecord.x,
        iprecord.y,
        iprecord.xi,
        iprecord.yi,
        iprecord.orientation,
        iprecord.scale,
        iprecord.interest,
        iprecord.polarity,
        iprecord.octave,
        iprecord.scale_lvl,
        iprecord.ndesc,
        iprecord
            .desc
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}

fn ipmatch_as_text(ipmatch: &IPMatch) -> String {
    let header = format!("{} {}", ipmatch.image_1.len(), ipmatch.image_2.len());
    let image_1_txt = ipmatch
        .image_1
        .iter()
        .map(|r| iprecord_as_text(r))
        .collect::<Vec<String>>()
        .join("\n");
    let image_2_txt = ipmatch
        .image_2
        .iter()
        .map(|r| iprecord_as_text(r))
        .collect::<Vec<String>>()
        .join("\n");
    vec![header, "\n".to_string(), image_1_txt, "\n".to_string(), image_2_txt].concat()
}

fn main() {
    let matches = App::new("info")
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
            println!("{}", ipmatch_as_text(&ipmatch));
        }
        Err(e) => {
            eprintln!("error: {:?}", e);
        }
    }
}
