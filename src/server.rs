use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
// use std::convert::TryInto;

use crate::http::{ParseError, Request};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {  
        Self {
            addr,
        }
    }

    pub fn run(self) { // take ownership here
        println!("listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // Result<T, E>

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // 1 k bytes

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}

