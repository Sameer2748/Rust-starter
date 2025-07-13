use std::fs;
use std::io::{prelude, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        println!("Connection established with ");
        handleStream(stream);
    }
}

fn handleStream(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // this is buffer of 1 kb
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line,filename) = 
       if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    println!("{}", filename);

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
