use http_server::header::CommonHeader;
use http_server::response::ResponseBuilder;
use http_server::server::Server;
use http_server::status_codes::HttpStatus;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

struct TestServer;

impl Server for TestServer {
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
}
#[tokio::main]
async fn main() -> io::Result<()> {
    let binded = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = binded.accept().await?;

        println!("{addr} Connected!");

        socket.readable().await?;
        let mut buf = [0u8; 1024];
        let _ = socket.try_read(&mut buf).unwrap();
        println! {"{}",String::from_utf8(buf.to_vec()).unwrap()};
        socket.writable().await?;
        socket.write(&TestServer::process_packet(&buf)).await?;
    }
}
