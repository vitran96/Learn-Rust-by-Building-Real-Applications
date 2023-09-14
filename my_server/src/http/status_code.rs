use std::fmt::Display;

// We need to impl Copy because status code live in stack
//   so we cannot move it to another variable
//   -> we need to copy it
// We need to impl Clone because we need to deep copy it
#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // "self as u16" will cast the reference to u16 -> wrong
        // "*self as u16" will cast the value to u16 -> correct
        // However *self requires that StatusCode implements Copy trait
        // and implement Copy trait requires that StatusCode implements Clone trait
        // Since we don't need special conversion or format, we can use derive(Copy, Clone)
        write!(f, "{}", *self as u16)
    }
}
