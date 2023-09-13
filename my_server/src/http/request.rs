use std::{
    error::Error,
    fmt::{Debug, Display},
    str::Utf8Error,
};

use super::{method::MethodError, Method, QueryString};

// We will switch from String to &str (string slice)
// Why?
// Because we don't need to own the data
// String will save the data to heap & allow us to manipulate the data
// &str will save the data to stack & we can't manipulate the data
// But a request should be immutable
// --
// &str requires lifetime annotation
// we are going to use
//
// #[] has no '!' so this attribute will only be applied to the line following it
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl Request<'_> {
    // Might fail
    // I don't remember what this function was for
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
    //     todo!()
    // }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // Parse data example:
    // Eg 1: GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(value: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        // Be default, set query_string to None
        // since we don't know if there is query_string or not
        let mut query_string = None;

        // Using 'match'
        // match path.find('?') {
        //     Some(i) => {
        //         // i+1 to skip '?'
        //         query_string = Some(&request[i + 1..]);

        //         // path of the request
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // Using 'unwrap'
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     // i+1 to skip '?'
        //     query_string = Some(&request[i + 1..]);

        //     // path of the request
        //     path = &path[..i];
        // }

        // Using 'if let'
        if let Some(i) = path.find('?') {
            // i+1 to skip '?'
            query_string = Some(QueryString::from(&path[i + 1..]));

            // path of the request
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string: query_string,
            method: method,
        })
    }
}

// this function only has 1 &str
// so by default, it will point to the same lifetime reference as the input
// however, if we have more parameters, we need to specify the lifetime reference for each parameter
//   because the compiler can't infer the lifetime reference
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

// Automatically convert MethodError to ParseError
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
