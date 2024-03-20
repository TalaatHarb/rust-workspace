use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    str,
};

#[derive(Debug)]
pub enum HttpProtocol {
    Http09,
    Http10,
    Http11,
    Http2,
    Http3,
}

#[derive(Debug)]
pub struct HttpHeader<'buf> {
    pub header_name: &'buf str,
    pub header_value: &'buf str,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    InvalidHeader,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidHeader => "InvalidHeader",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}

pub fn get_parts(buffer: &[u8]) -> (Vec<&[u8]>, usize) {
    let mut result: Vec<&[u8]> = Vec::new();

    let length = buffer.len();
    let mut last_find_end = 0;

    for i in 0..(length - 1) {
        if buffer[i] == 13 && buffer[i + 1] == 10 {
            if last_find_end == i {
                while buffer[last_find_end] != 0 && last_find_end < length {
                    last_find_end += 1;
                }
                result.push(&buffer[i + 2..last_find_end]);
                break;
            }
            result.push(&buffer[last_find_end..i]);
            last_find_end = i + 2;
        }
    }

    return (result, last_find_end);
}

pub fn get_next_word(string: &str) -> Option<(&str, &str)> {
    for (i, c) in string.chars().enumerate() {
        if c == ' ' {
            return Some((&string[..i], &string[i + 1..]));
        }
    }

    return None;
}

pub fn get_headers<'buf>(parts: &Vec<&'buf [u8]>) -> Result<Vec<HttpHeader<'buf>>, ParseError> {
    let length: usize = parts.len();
    let mut headers: Vec<HttpHeader<'buf>> = Vec::new(); // can end up empty

    let mut i: usize = 1; // skip first line because it is parsed on its own
    while i < length - 1 {
        headers.push(get_header(
            str::from_utf8(parts[i]).or(Err(ParseError::InvalidEncoding))?,
        )?);
        i += 1;
    }
    return Ok(headers);
}

fn get_header(full_header: &str) -> Result<HttpHeader, ParseError> {
    let i: Option<usize> = full_header.find(":");

    match i {
        Some(location) => {
            return Ok(HttpHeader {
                header_name: &full_header[0..location],
                header_value: &full_header[location + 1..], // TODO skip leading whitespace
            });
        }
        None => return Err(ParseError::InvalidHeader),
    }
}
