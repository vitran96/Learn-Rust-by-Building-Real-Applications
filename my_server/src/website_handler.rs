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

        // This will cause security issue
        // because if path contains "../" it will go up the directory
        // and hacker can access all of our server files

        // we can fix this security error by checking the real path
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    // path is safe
                    // ok() will convert Result<T, E> to Option<T>
                    // very handy
                    fs::read_to_string(path).ok()
                } else {
                    // path is not safe
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
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
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
