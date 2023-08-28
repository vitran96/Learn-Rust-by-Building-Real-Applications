enum Method {
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

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

fn main() {
    // Start the server
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
