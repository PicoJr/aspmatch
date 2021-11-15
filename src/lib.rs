//! # aspmatch
//!
//! Parse [ASP match files](https://stereopipeline.readthedocs.io/en/latest/outputfiles.html?highlight=match#guide-to-output-files)
//!
//! ## Example
mod data;
mod parser;

pub use data::{IPMatch, IPRecord};

pub use parser::{
    dump_match_as_binary, dump_match_as_binary_to_file, dump_match_as_binary_to_path,
    dump_match_as_text, dump_match_as_text_to_file, dump_match_as_text_to_path, ipmatch,
    ipmatch_text, iprecord, iprecord_text, parse_binary_match_file, parse_binary_match_file_path,
    parse_text_match_file, parse_text_match_file_path, ASPMatchError,
};
