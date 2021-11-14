//! # aspmatch
//!
//! Parse [ASP match files](https://stereopipeline.readthedocs.io/en/latest/outputfiles.html?highlight=match#guide-to-output-files)
//!
//! ## Example
mod data;
mod parser;

pub use data::{IPMatch, IPRecord};

pub use parser::{
    dump_match_file, dump_match_file_path, ipmatch, iprecord, parse_match_file,
    parse_match_file_path,
};
