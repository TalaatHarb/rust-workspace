use crate::http::*;
use std::{
    convert::TryFrom,
    str,
};

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
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub query_string: Option<String>,
    pub protocol: HttpProtocol,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
}
impl TryFrom<&[u8]> for HttpRequest {
    type Error = ParseError;
    fn try_from(buffer: &[u8]) -> Result<Self, <Self as TryFrom<&[u8]>>::Error> {
        let parts = get_parts(buffer);
        if parts.len() < 1 {
            return Err(ParseError::InvalidRequest);
        }

        let first_line = str::from_utf8(&parts[0]).or(Err(ParseError::InvalidEncoding))?;
        let (method_string, rest_of_request_line) =
            get_next_word(first_line).ok_or(ParseError::InvalidRequest)?;
        let (complete_path, protocol_string) =
            get_next_word(rest_of_request_line).ok_or(ParseError::InvalidRequest)?;

        let path_parts: Vec<&str> = complete_path.split("?").collect();

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

        let protocol: HttpProtocol = match protocol_string {
            "HTTP/0.9" | "" => HttpProtocol::Http09,
            "HTTP/1.0" => HttpProtocol::Http10,
            "HTTP/1.1" => HttpProtocol::Http11,
            "HTTP/2" => HttpProtocol::Http2,
            "HTTP/3" => HttpProtocol::Http3,
            _ => return Err(ParseError::InvalidProtocol),
        };

        let (headers, body) = get_headers_and_body(&parts).or(Err(ParseError::InvalidRequest))?; // TODO fix body parsing

        return Ok(HttpRequest {
            method: method,
            path: path_parts[0].to_string(),
            query_string: match path_parts.len() {
                1 => None,
                2 => Some(path_parts[1].to_string()),
                _ => return Err(ParseError::InvalidRequest),
            },
            protocol: protocol,
            headers: headers,
            body: body,
        });
    }
}
