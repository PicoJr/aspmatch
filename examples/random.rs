use aspmatch::{dump_match_file_path, IPMatch, IPRecord};
use clap::{crate_authors, crate_version, App, Arg};
use rand::prelude::*;
use std::path::PathBuf;

extern crate clap;

fn random_iprecord() -> IPRecord {
    let mut rng = thread_rng();
    let ndesc: u64 = rng.gen_range(1..=4);
    IPRecord {
        x: rng.gen(),
        y: rng.gen(),
        xi: rng.gen_range(-100..=100),
        yi: rng.gen_range(-100..=100),
        orientation: rng.gen(),
        scale: rng.gen(),
        interest: rng.gen(),
        polarity: rng.gen(),
        octave: rng.gen_range(0..=100),
        scale_lvl: rng.gen_range(0..=100),
        ndesc,
        desc: vec![rng.gen(); ndesc as usize],
    }
}

fn random_ipmatch() -> IPMatch {
    let mut rng = thread_rng();
    let size_1: usize = rng.gen_range(1..=5);
    let records_1: Vec<IPRecord> = (1..=size_1).map(|_| random_iprecord()).collect();
    let size_2: usize = rng.gen_range(1..=5);
    let records_2: Vec<IPRecord> = (1..=size_2).map(|_| random_iprecord()).collect();
    IPMatch {
        image_1: records_1,
        image_2: records_2,
    }
}

fn main() {
    let matches = App::new("random")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("dump random match file to disk")
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT")
                .help("match file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let output_file = matches.value_of("output").expect("OUTPUT is required");
    let ipmatch = random_ipmatch();
    match dump_match_file_path(&ipmatch, PathBuf::from(output_file)) {
        Ok(_) => {
            println!("random ipmatch written to {}", output_file);
        }
        Err(e) => {
            eprintln!("error: {:?}", e)
        }
    }
}
