use super::StatusCode;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
        }
    }

    //pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {  // dynamic dispatch, run time cost
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {  // static dispatch, no run time cost
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, self.status_code.reason_phrase(), body)
    }

    // compiler will create below code if the program 
    // have two send with concrete Write: TcpStream, File:
    // pub fn send_TcpStream(&self, stream: &mut TcpStream) -> IoResult<()> {copy the body here..}
    // pub fn send_File(&self, stream: &mut File) -> IoResult<()> {copy the body here..}
}


