use std::fs;

use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path: public_path,
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // ok() will convert Result<T, E> to Option<T>
        // very handy
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::Ok, Some("<h1>Hello world!</h1>".to_string()))
        match request.method() {
            // Handle Routing for method
            // Handling GET method
            Method::GET => match request.path() {
                // "/" => Response::new(StatusCode::Ok, Some("<h1>Hello world!</h1>".to_string())),
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/welcome" => Response::new(StatusCode::Ok, Some("<h1>Welcome!</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
