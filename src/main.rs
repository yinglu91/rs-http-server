#![allow(dead_code)]
use server::Server;

mod http;
mod server;

fn main() {
    let http_server = Server::new("127.0.0.1:8080".to_string());
    http_server.run();
}
   
/* request:
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

// &str -- immutable, string slice, is part of the string, is a view inside the string

// mod server; eq to
// namespace
// mod server {
//  {content of file go here} 
// }

// http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/

// https://code.visualstudio.com/docs/editor/debugging

// https://github.com/gavadinov/Learn-Rust-by-Building-Real-Applications/blob/master/server/src/main.rs
