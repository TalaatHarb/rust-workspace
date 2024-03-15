use crate::http::*;
use std::{convert::TryFrom, str};

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

#[derive(Debug)]
pub struct HttpRequest<'buf> {
    pub size: usize,
    pub method: HttpMethod,
    pub path: &'buf str,
    pub query_string: Option<&'buf str>,
    pub protocol: HttpProtocol,
    pub headers: Vec<HttpHeader<'buf>>,
    pub body: &'buf [u8],
}

impl<'buf> TryFrom<&'buf [u8]> for HttpRequest<'buf> {
    type Error = ParseError;
    fn try_from(buffer: &'buf [u8]) -> Result<Self, <Self as TryFrom<&'buf [u8]>>::Error> {
        let (parts, n) = get_parts(buffer);
        if parts.len() < 1 {
            return Err(ParseError::InvalidRequest);
        }

        let first_line: &str = str::from_utf8(&parts[0]).or(Err(ParseError::InvalidEncoding))?;
        let (method_string, rest_of_request_line) =
            get_next_word(first_line).ok_or(ParseError::InvalidRequest)?;
        let (complete_path, protocol_string) =
            get_next_word(rest_of_request_line).ok_or(ParseError::InvalidRequest)?;

        let mut path_parts: Vec<&'buf str> = Vec::new();
        if let Some(i) = complete_path.find('?') {
            path_parts.push(&complete_path[..i]);
            path_parts.push(&complete_path[i + 1..]);
        } else {
            path_parts.push(complete_path);
        }

        let method: HttpMethod = parse_method(method_string).or(Err(ParseError::InvalidMethod))?;
        let protocol: HttpProtocol =
            parse_protocol(protocol_string).or(Err(ParseError::InvalidProtocol))?;
        let headers = get_headers(&parts).or(Err(ParseError::InvalidRequest))?;

        return Ok(HttpRequest::<'buf> {
            size: n,
            method: method,
            path: path_parts[0],
            query_string: match path_parts.len() {
                1 => None,
                2 => Some(path_parts[1]),
                _ => return Err(ParseError::InvalidRequest),
            },
            protocol: protocol,
            headers: headers,
            body: parts[parts.len() - 1],
        });
    }
}

fn parse_protocol(protocol_string: &str) -> Result<HttpProtocol, ParseError> {
    let protocol: HttpProtocol = match protocol_string {
        "HTTP/0.9" | "" => HttpProtocol::Http09,
        "HTTP/1.0" => HttpProtocol::Http10,
        "HTTP/1.1" => HttpProtocol::Http11,
        "HTTP/2" => HttpProtocol::Http2,
        "HTTP/3" => HttpProtocol::Http3,
        _ => return Err(ParseError::InvalidProtocol),
    };
    Ok(protocol)
}

fn parse_method(method_string: &str) -> Result<HttpMethod, ParseError> {
    let method: HttpMethod = match method_string {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "PATCH" => HttpMethod::PATCH,
        "DELETE" => HttpMethod::DELETE,
        "HEAD" => HttpMethod::HEAD,
        "OPTIONS" => HttpMethod::OPTIONS,
        "CONNECT" => HttpMethod::CONNECT,
        "TRACE" => HttpMethod::TRACE,
        _ => return Err(ParseError::InvalidMethod),
    };
    Ok(method)
}
