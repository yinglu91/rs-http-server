use std::net::TcpListener;
use std::io::{Write, Read};
use std::convert::TryFrom;

use crate::http::{Request, Response, StatusCode};

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

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("{:?}", request); 
                                    Response::new(StatusCode::Ok, 
                                        Some("<h1>It work</h1>".to_string()))
                                },
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                },
                            };

                            if let Err(e) = response.send(&mut  stream) {
                                println!("Failed to send response: {}", e);
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
// GET 127.0.0.1:8080/search?a=10
// body json:
// {
//     "name":"Ying Lu"
// }

// Finished dev [unoptimized + debuginfo] target(s) in 0.98s
// Running `target\debug\http-server.exe`
// listening on 127.0.0.1:8080
// Received a request: GET /search?a=10 HTTP/1.1
// Content-Type: application/json
// User-Agent: PostmanRuntime/7.26.8
// Accept: */*
// Cache-Control: no-cache
// Postman-Token: 87cf84e7-11e7-4f46-bf6a-48bf2c633b77
// Host: 127.0.0.1:8080
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Content-Length: 26

// {
// "name":"Ying Lu"
// }
// Request { path: "/search", query_string: Some(QueryString { data: {"a": Single("10")} }), method: GET }


// change:
// GET 127.0.0.1:8080/search
// Request { path: "/search", query_string: None, method: GET }
// GET 127.0.0.1:8080/search?a=10&a=20
// output:
// Request { path: "/search", query_string: Some(QueryString { data: {"a": Multiple(["10", "20"])} }), method: GET }