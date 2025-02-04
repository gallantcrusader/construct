use std::collections::VecDeque;

use super::{header::CommonHeader, status_codes::HttpStatus};

pub struct ResponseBuilder {
    pub status: HttpStatus,
    pub common_headers: VecDeque<(CommonHeader, Vec<u8>)>,
    pub more_headers: VecDeque<(Vec<u8>, Vec<u8>)>,
    pub body: Option<Vec<u8>>,
}

impl ResponseBuilder {
    pub fn new(status: HttpStatus) -> ResponseBuilder {
        ResponseBuilder {
            status,
            common_headers: VecDeque::new(),
            more_headers: VecDeque::new(),
            body: None,
        }
    }

    pub fn build(&self) -> Vec<u8> {
        let mut response = Vec::<u8>::new();
        response.extend_from_slice(b"HTTP/1.1 ");
        response.extend_from_slice(&*self.status.as_bytes());

        for (header, body) in self.common_headers.iter() {
            response.extend_from_slice(&*header.as_bytes(&*body));
        }

        for (header, body) in self.more_headers.iter() {
            response.extend_from_slice(&*header);
            response.extend_from_slice(&*body);
        }

        response.extend_from_slice(b"\r\n");

        if let Some(txt) = self.body.clone() {
            response.extend_from_slice(&*txt);
        }

        response
    }
}
