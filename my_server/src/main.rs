use http::request::Request;
use server::Server;

mod http {
    pub mod method {
        pub enum Method {
            GET,
            POST,
            PUT,
            DELETE,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }

    pub mod request {
        use super::method::Method;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }
    }
}

mod server {
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
            println!("Listening on {}", self.addr);
        }
    }
}

fn main() {
    // Start the server
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
