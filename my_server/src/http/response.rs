use std::{
    fmt::{Display, Formatter},
    io::Write,
    net::TcpStream,
};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code: status_code,
            body: body,
        }
    }

    // This is an OK way to send response, but it is not generic
    // pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {

    // This one will compile, but might not work at runtime
    // pub fn send(&self, stream: &mut Write) -> std::io::Result<()> {

    // This one will create an overhead at runtime, because it will try to look up for Write trait implementation at runtime
    // pub fn send(&self, stream: &mut dyn Write) -> std::io::Result<()> {

    // This one will make compilation slower and artifact bigger,
    // because it will lookup Write trail implementation and implement this function twice at compile time
    pub fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "HTTP/1.1 {} {}\r\n\r\n{}",
//             self.status_code,
//             self.status_code.reason_phrase(),
//             self.body.clone().unwrap_or(String::from(""))
//         )
//     }
// }
