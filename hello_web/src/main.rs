
use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    println!("-------------开始打印---------------");
    println!("{}", String::from_utf8_lossy(&buffer[..]));
    println!("-------------结束打印---------------");
    println!("\n\n\n\n\n\n");

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "resources/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "resources/404.html")
    };

    let response = format!("{}{}", status_line, fs::read_to_string(file_name).unwrap());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
