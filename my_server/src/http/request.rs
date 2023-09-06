use std::{
    error::Error,
    fmt::{Debug, Display},
    result,
    str::Utf8Error,
};

use super::Method;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    /**
     * Might fail
     */
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // Parse data example:
    // Eg 1: GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // Basic validation
        // match std::str::from_utf8(value) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // Alternative to above match => too complicated
        // match std::str::from_utf(value).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        // Alternative to above match => look ok
        // This will wrap ParseError::InvalidEncoding
        // let request = std::str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?;

        // Alternative to above match => this will wrap any error
        let request = std::str::from_utf8(value)?;

        // Next, we need to parse the request

        // request must be sliced to removed parsed part
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (query_string, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse().or(Err(ParseError::InvalidMethod))?;

        todo!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // result: begin -> current index
            let result = &request[..i];

            // unused_part: current index -> end
            // then trim() to remove leading and trailing whitespaces -> not efficient but safer than trying to manipulate index
            let unused_part = &request[i..].trim();
            return Some((result, unused_part));
        }
    }

    None
}

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

// OPTIONAL
// But we should implement this for our custom error type
impl Error for ParseError {}

// REQUIRED to implement Error trait to ParseError
impl Display for ParseError {
    // We can use "use std::fmt::Result as FmtResult;" to avoid used of name "Result"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

// REQUIRED to implement Error trait to ParseError
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

// Automatically convert Utf8Error to ParseError
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
