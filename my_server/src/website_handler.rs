use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::Ok, Some("<h1>Hello world!</h1>".to_string()))
        match request.method() {
            // Handle Routing for method
            // Handling GET method
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Hello world!</h1>".to_string())),
                "/welcome" => Response::new(StatusCode::Ok, Some("<h1>Welcome!</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
