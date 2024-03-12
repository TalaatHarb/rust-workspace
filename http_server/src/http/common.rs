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
pub struct HttpHeader {
    pub header_name: String,
    pub header_value: String,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}

pub fn get_parts(buffer: &[u8]) -> Vec<&[u8]> {
    let mut result: Vec<&[u8]> = Vec::new();

    let length = buffer.len();
    let mut last_find_end = 0;

    for i in 0..(length - 1) {
        if buffer[i] == 13 && buffer[i + 1] == 10 {
            result.push(&buffer[last_find_end..i]);
            last_find_end = i + 2;
        }
    }

    return result;
}

pub fn get_next_word(string: &str) -> Option<(&str, &str)> {
    for (i, c) in string.chars().enumerate() {
        if c == ' ' {
            return Some((&string[..i], &string[i + 1..]));
        }
    }

    return None;
}

pub fn get_headers_and_body(
    parts: &Vec<&[u8]>,
) -> Result<(Vec<HttpHeader>, Option<Vec<u8>>), ParseError> {
    let length = parts.len();
    let mut headers: Vec<HttpHeader> = Vec::new(); // can end up empty

    let mut i: usize = 1; // skip first line because it is parsed on its own
    while i < length {
        if parts[i] == [] {
            i += 1;
            break;
        }

        headers.push(get_header(
            str::from_utf8(parts[i]).or(Err(ParseError::InvalidEncoding))?,
        ));
        i += 1;
    }

    let body: Option<Vec<u8>> = match i {
        n if n == length => None,
        _ => Some(parts[i..].iter().flat_map(|s| s.iter().copied()).collect()),
    };

    return Ok((headers, body));
}

fn get_header(full_header: &str) -> HttpHeader {
    let parts: Vec<&str> = full_header.split(":").collect();

    return HttpHeader{
        header_name: parts[0].to_string(),
        header_value: parts[1].to_string()
    };
}
