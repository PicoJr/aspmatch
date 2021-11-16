//! # aspmatch
//!
//! Parse [ASP match files](https://stereopipeline.readthedocs.io/en/latest/outputfiles.html?highlight=match#guide-to-output-files)
//!
//! ## Example
//!
//! Dump/Parse match as binary.
//!
//! ```
//! use tempfile::tempfile;
//! use std::io::{Write};
//! use std::io::Seek;
//! use aspmatch::{IPMatch, dump_match_as_binary, parse_binary_match_file};
//! # use aspmatch::ASPMatchError;
//! # use std::io::SeekFrom;
//!
//! # fn main() {
//! #     if let Err(_) = run() {
//! #         ::std::process::exit(1);
//! #     }
//! # }
//! # fn run() -> Result<(), ASPMatchError> {
//! // Create a file inside of `std::env::temp_dir()`.
//! let mut tmpfile = tempfile::tempfile().unwrap();
//! let ipmatch = IPMatch::default();
//!
//! // write match as binary to file
//! dump_match_as_binary(&ipmatch, &mut tmpfile)?;
//! tmpfile.seek(SeekFrom::Start(0)).unwrap();
//!
//! // parse match from binary file
//! let parsed = parse_binary_match_file(&tmpfile)?;
//! assert_eq!(ipmatch, parsed);
//! # Ok(())
//! # }
//! ```
//!
//! Dump/Parse match as text.
//!
//! ```
//! use tempfile::tempfile;
//! use std::io::{Write};
//! use std::io::Seek;
//! use aspmatch::{IPMatch, dump_match_as_text, parse_text_match_file};
//! # use aspmatch::ASPMatchError;
//! # use std::io::SeekFrom;
//!
//! # fn main() {
//! #     if let Err(_) = run() {
//! #         ::std::process::exit(1);
//! #     }
//! # }
//! # fn run() -> Result<(), ASPMatchError> {
//! // Create a file inside of `std::env::temp_dir()`.
//! let mut tmpfile = tempfile::tempfile().unwrap();
//! let ipmatch = IPMatch::default();
//!
//! // write match as text to file
//! dump_match_as_text(&ipmatch, &mut tmpfile)?;
//! tmpfile.seek(SeekFrom::Start(0)).unwrap();
//!
//! // parse match from text file
//! let parsed = parse_text_match_file(&tmpfile)?;
//! assert_eq!(ipmatch, parsed);
//! # Ok(())
//! # }
//! ```
mod data;
mod parser;

pub use data::{IPMatch, IPRecord};

pub use parser::{
    dump_match_as_binary, dump_match_as_binary_to_file, dump_match_as_binary_to_path,
    dump_match_as_text, dump_match_as_text_to_file, dump_match_as_text_to_path, ipmatch,
    ipmatch_text, iprecord, iprecord_text, parse_binary_match_file, parse_binary_match_file_path,
    parse_text_match_file, parse_text_match_file_path, ASPMatchError,
};
