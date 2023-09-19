// Disabling compiler dead code warnings
// #![] has '!' so this attribute will apply to this item
// and also sub-item
// #![allow(dead_code)]

use crate::http::{Response, StatusCode};
// Must go from root. We use "crate::" instead of "super::"
use crate::http::request::Request;
use std::convert::TryFrom;
use std::io::Write;
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    pub fn run(self) {
        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        loop {
            let res = listener.accept();
            match res {
                // stream:TcpStream,
                Ok((mut stream, _)) => {
                    println!("Connection established");
                    // Byte slice?
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                            // must convert to &[u8; 1024]
                            // Request::try_from(&buf[..]);
                            // let res: &Result<Request, _> = &buf.try_into();
                            let result = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    // buf is borrowed by request.
                                    // so we can't use buf anymore
                                    // Eg: bug[0] = 0; is not allowed
                                    dbg!(request);

                                    // let response =
                                    //     Response::new(crate::http::StatusCode::NotFound, None);

                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>Hello world!</h1>".to_string()),
                                    )

                                    // Doing this, we will create new allocation
                                    // write!(stream, "{}", response);
                                }
                                Err(e) => {
                                    println!("Failed to parse a request:  {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            // Send and Check error here
                            if let Err(e) = result.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
