use std::{
    error::Error,
    fmt::{Debug, Display},
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
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// OPTIONAL
// But we should implement this for our custom error type
impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    // fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {}
}

// REQUIRED to implement Error trait to ParseError
impl Display for ParseError {
    // We can use "use std::fmt::Result as FmtResult;" to avoid used of name "Result"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// REQUIRED to implement Error trait to ParseError
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest => write!(f, "InvalidRequest"),
            Self::InvalidEncoding => write!(f, "InvalidEncoding"),
            Self::InvalidProtocol => write!(f, "InvalidProtocol"),
            Self::InvalidMethod => write!(f, "InvalidMethod"),
        }
    }
}
