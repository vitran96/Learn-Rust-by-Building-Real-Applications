use std::{
    error::Error,
    net::{SocketAddr, TcpListener, TcpStream},
};

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
                Ok((stream, _)) => {
                    println!("Ok");
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
