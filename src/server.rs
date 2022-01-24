use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;

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
                    let mut buffer = [0; 1024]; // 1k bytes

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {println!("Request: {:?}", request); },
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


// test from postman:
// GET 127.0.0.1:8080/search?a=1088
// body json:
// {
//     "name":"Ying Lu"
// }

// Finished dev [unoptimized + debuginfo] target(s) in 0.95s 
// Running `target\debug\http-server.exe`
// listening on 127.0.0.1:8080
// Received a request: GET /search?a=1088 HTTP/1.1
// Content-Type: application/json
// User-Agent: PostmanRuntime/7.26.8
// Accept: */*
// Cache-Control: no-cache
// Postman-Token: 53afb4a4-d24c-451d-adcd-674d715cf237
// Host: 127.0.0.1:8080
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Content-Length: 26

// {
//     "name":"Ying Lu"
// }
// Request: Request { path: "/search", query_string: Some("a=1088"), method: GET }


