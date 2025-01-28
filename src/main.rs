mod comp;
use crate::comp::{header::CommonHeader, status_codes::HttpStatus};

use std::collections::VecDeque;
use std::io;

use tokio::{io::AsyncWriteExt, net};

pub struct ResponseBuilder {
    status: HttpStatus,
    common_headers: VecDeque<(CommonHeader, Vec<u8>)>,
    more_headers: VecDeque<(Vec<u8>, Vec<u8>)>,
    body: Option<Vec<u8>>,
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
        response.extend_from_slice(&*self.status.to_bytes());

        for (header, body) in self.common_headers.iter() {
            response.extend_from_slice(&*header.to_bytes(&*body));
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

fn process_packet(_: &[u8]) -> Vec<u8> {
    let mut builder = ResponseBuilder::new(HttpStatus::Ok);
    builder
        .common_headers
        .push_back((CommonHeader::Server, b"Crude Server".to_vec()));
    builder
        .common_headers
        .push_back((CommonHeader::ContentType, b"text/html".to_vec()));

    builder.body =
        Some(b"<html>\n\t<body>\n\t\t<h1>Request received!</h1>\n\t<body>\n</html>".to_vec());

    let response = builder.build();
    response
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let binded = net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = binded.accept().await?;

        println!("{addr} Connected!");

        socket.readable().await?;
        let mut buf = [0u8; 1024];
        let pcks = socket.try_read(&mut buf).unwrap();
        println! {"{pcks}"};

        socket.writable().await?;
        socket.write(&process_packet(&buf)).await?;
    }
}
