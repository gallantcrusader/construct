use std::collections::VecDeque;

use super::status_codes::HttpStatus;

pub struct ResponseBuilder {
    status: HttpStatus,
    headers: VecDeque<(Vec<u8>, Vec<u8>)>,
    pub body: Option<Vec<u8>>,
}

impl ResponseBuilder {
    pub fn new(status: HttpStatus) -> ResponseBuilder {
        ResponseBuilder {
            status,
            headers: VecDeque::new(),
            body: None,
        }
    }

    pub fn build(&self) -> Vec<u8> {
        let mut response = Vec::<u8>::new();
        response.extend_from_slice(b"HTTP/1.1 ");
        response.extend_from_slice(&*self.status.as_bytes());

        for (header, body) in self.headers.iter() {
            response.extend_from_slice(&header);
            response.extend_from_slice(&body);
            response.extend_from_slice(b"\r\n");
        }

        response.extend_from_slice(b"\r\n");

        if let Some(txt) = self.body.clone() {
            response.extend_from_slice(&*txt);
        }

        response
    }

    pub fn add_header(&mut self, header: Vec<u8>, body: Vec<u8>) {
        // println!(
        //     "HEADER TITLE: {}",
        //     String::from_utf8(header.clone()).unwrap()
        // );
        // println!("HEADER BODY: {}", String::from_utf8(body.clone()).unwrap());
        self.headers.push_back((header, body));
    }
}
