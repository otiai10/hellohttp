use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established!!");
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap(); 
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, HTTP!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
