use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    println!("Starting web server...");

    let listener = TcpListener::bind("0.0.0.0:8090").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Got connection request");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    println!("Got content {}", contents.len());

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}