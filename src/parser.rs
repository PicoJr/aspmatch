use crate::data::{IPMatch, IPRecord};
use nom::multi::{count, separated_list0};
use nom::number::complete::{le_f32, le_i32, le_u32, le_u64, le_u8};
use nom::sequence::{tuple};
use nom::IResult;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use nom::character::complete::space1;
use nom::combinator::verify;
use thiserror::Error;

/// Custom Error Enum for lib consumption.
#[derive(Error, Debug)]
pub enum ASPMatchError {
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("parser error")]
    Parser(#[from] nom::Err<nom::error::Error<Vec<u8>>>),
}

pub fn iprecord_text(input: &str) -> IResult<&str, IPRecord> {
    let (i, (x, _, y, _)) = tuple((
        nom::number::complete::float,
        space1, // x
        nom::number::complete::float,
        space1, // y
    ))(input)?;
    let (i, (xi, _, yi, _)) = tuple((
        nom::character::complete::i32,
        space1, // xi
        nom::character::complete::i32,
        space1, // yi
    ))(i)?;
    let (i, (orientation, _, scale, _, interest, _)) = tuple((
        nom::number::complete::float,
        space1, // orientation
        nom::number::complete::float,
        space1, // scale
        nom::number::complete::float,
        space1, // interest
    ))(i)?;
    let (i, (polarity, _)) = tuple((
        nom::character::complete::u8,
        space1, // polarity
    ))(i)?;
    let (i, (octave, _, scale_lvl, _)) = tuple((
        nom::character::complete::u32,
        space1, // octave
        nom::character::complete::u32,
        space1, // scale_lvl
    ))(i)?;
    let (i, (ndesc, _)) = tuple((
        nom::character::complete::u64,
        space1, // ndesc
    ))(i)?;
    let (i, desc) = verify(
        separated_list0(space1, nom::number::complete::float),
        |v: &[f32]| v.len() == ndesc as usize,
    )(i)?;

    Ok((
        i,
        IPRecord {
            x,
            y,
            xi,
            yi,
            orientation,
            scale,
            interest,
            polarity,
            octave,
            scale_lvl,
            ndesc,
            desc,
        },
    ))
}

/// Parse IPRecord from byte slice assuming little endianness
///
/// Returns remaining bytes
///
/// ```
/// use aspmatch::IPRecord;
/// use aspmatch::iprecord;
/// let record = IPRecord::default();
/// let bytes = record.as_le_bytes();
/// let (remaining_bytes, parsed) = iprecord(bytes.as_slice()).unwrap();
/// assert!(remaining_bytes.is_empty());
/// assert_eq!(parsed, record);
/// ```
pub fn iprecord(input: &[u8]) -> IResult<&[u8], IPRecord> {
    let (i, (x, y)) = tuple((le_f32, le_f32))(input)?;
    let (i, (xi, yi)) = tuple((le_i32, le_i32))(i)?;
    let (i, (orientation, scale, interest)) = tuple((le_f32, le_f32, le_f32))(i)?;
    let (i, polarity) = le_u8(i)?;
    let (i, (octave, scale_lvl)) = tuple((le_u32, le_u32))(i)?;
    let (i, ndesc) = le_u64(i)?;
    let (i, desc) = count(le_f32, ndesc as usize)(i)?;
    Ok((
        i,
        IPRecord {
            x,
            y,
            xi,
            yi,
            orientation,
            scale,
            interest,
            polarity,
            octave,
            scale_lvl,
            ndesc,
            desc,
        },
    ))
}

pub fn ipmatch_text(input: &str) -> IResult<&str, IPMatch> {
    let (i, (size_1, _, size_2, _)) = tuple((
        nom::character::complete::u64,
        nom::character::complete::space1,
        nom::character::complete::u64,
        nom::character::complete::line_ending)
    )(input)?;
    let (i, ip_records) = verify(
        separated_list0(
            nom::character::complete::line_ending,
            iprecord_text
        ),
        |v: &[IPRecord]| v.len() == (size_1 + size_2) as usize
    )(i)?;
    let (image_1_ip_records, image_2_ip_records) = ip_records.split_at(size_1 as usize);
    Ok((i, IPMatch {
        image_1: image_1_ip_records.to_vec(),
        image_2: image_2_ip_records.to_vec(),
    }))
}

/// Parse IPMatch from byte slice assuming little endianness
///
/// Returns remaining bytes
///
/// ```
/// use aspmatch::IPMatch;
/// use aspmatch::ipmatch;
/// let _match = IPMatch::default();
/// let bytes = _match.as_le_bytes();
/// let (remaining_bytes, parsed) = ipmatch(bytes.as_slice()).unwrap();
/// assert!(remaining_bytes.is_empty());
/// assert_eq!(parsed, _match);
/// ```
pub fn ipmatch(input: &[u8]) -> IResult<&[u8], IPMatch> {
    let (i, (size_1, size_2)) = tuple((le_u64, le_u64))(input)?;
    let (i, image_1_ip_records) = count(iprecord, size_1 as usize)(i)?;
    let (i, image_2_ip_records) = count(iprecord, size_2 as usize)(i)?;
    Ok((
        i,
        IPMatch {
            image_1: image_1_ip_records,
            image_2: image_2_ip_records,
        },
    ))
}

/// Parse IPMatch from file assuming file fits in RAM
pub fn parse_match_file(match_file: &File) -> Result<IPMatch, ASPMatchError> {
    let mut buf_reader = BufReader::new(match_file);
    let mut buf = vec![];
    buf_reader.read_to_end(&mut buf)?;
    let (_, m) = ipmatch(&buf).map_err(|e| ASPMatchError::Parser(e.to_owned()))?;
    Ok(m)
}

/// Parse IPMatch from file at path, assuming file fits in RAM
pub fn parse_match_file_path<P: AsRef<Path>>(path: P) -> Result<IPMatch, ASPMatchError> {
    let match_file = File::open(path)?;
    parse_match_file(&match_file)
}

/// Dump IPMatch to file
pub fn dump_match_file(ipmatch: &IPMatch, match_file: &mut File) -> Result<(), ASPMatchError> {
    match_file.write_all(ipmatch.as_le_bytes().as_slice())?;
    Ok(())
}

/// Dump IPMatch to file at path
pub fn dump_match_file_path<P: AsRef<Path>>(
    ipmatch: &IPMatch,
    path: P,
) -> Result<(), ASPMatchError> {
    let mut match_file = OpenOptions::new().write(true).create(true).open(path)?;
    dump_match_file(ipmatch, &mut match_file)
}

#[cfg(test)]
mod tests {
    use crate::data::{IPMatch, IPRecord};
    use crate::parser::{ipmatch, iprecord, iprecord_text, ipmatch_text};
    use crate::{dump_match_file, parse_match_file};
    use std::io::{Seek, SeekFrom};

    fn dummy_iprecord() -> IPRecord {
        let ndesc = 2;
        IPRecord {
            x: 42.0,
            y: 43.0,
            xi: 42,
            yi: 43,
            orientation: 52.0,
            scale: 53.0,
            interest: 54.0,
            polarity: 3,
            octave: 62,
            scale_lvl: 63,
            ndesc,
            desc: vec![44.0; ndesc as usize],
        }
    }

    fn dummy_ipmatch() -> IPMatch {
        IPMatch {
            image_1: vec![dummy_iprecord()],
            image_2: vec![dummy_iprecord(), dummy_iprecord()],
        }
    }

    #[test]
    fn test_iprecord() {
        let expected = dummy_iprecord();
        let input: Vec<u8> = expected.as_le_bytes();
        let (i, iprecord) = iprecord(&input).unwrap();
        assert!(i.is_empty());
        assert_eq!(iprecord, expected);
    }

    #[test]
    fn test_iprecord_text() {
        let expected = dummy_iprecord();
        let input = expected.as_text();
        let (i, iprecord) = iprecord_text(&input).unwrap();
        assert!(i.is_empty());
        assert_eq!(iprecord, expected);
    }

    #[test]
    fn test_ipmatch() {
        let expected = dummy_ipmatch();
        let input: Vec<u8> = expected.as_le_bytes();
        let (i, ipmatch) = ipmatch(&input).unwrap();
        assert!(i.is_empty());
        assert_eq!(ipmatch, expected);
    }

    #[test]
    fn test_ipmatch_text() {
        let expected = dummy_ipmatch();
        let input = expected.as_text();
        let (i, ipmatch) = ipmatch_text(&input).unwrap();
        assert!(i.is_empty());
        assert_eq!(ipmatch, expected);
    }

    #[test]
    fn test_dump_and_parse_match_file() {
        let expected = dummy_ipmatch();
        let mut tmpfile = tempfile::tempfile().unwrap();
        let dump = dump_match_file(&expected, &mut tmpfile);
        assert!(dump.is_ok());
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let parse = parse_match_file(&tmpfile);
        println!("{:?}", parse);
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap(), expected);
    }
}
