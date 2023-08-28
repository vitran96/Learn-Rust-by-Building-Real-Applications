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
