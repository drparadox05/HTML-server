#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                // mutable borrow of stream is more appropriate here because reading a stream changes its internal state
                let mut reader = BufReader::new(&mut stream);
                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let parts : Vec<&str> = request_line.trim_end().split_whitespace().collect();
                let path = parts.get(1).unwrap_or(&"");

                let response = if *path == "/" {
                    "HTTP/1.1 200 OK\r\n\r\n"
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n"
                };

                let response = "HTTP/1.1 200 OK\r\n\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    println!("Failed to write to stream: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
