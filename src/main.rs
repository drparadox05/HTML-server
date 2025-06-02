#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};

/*
The two headers (content type & content length) are required for the client to be able to parse the response body. 
Note that each header ends in a CRLF, and the entire header section also ends in a CRLF.
*/


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                /*
                 mutable borrow of stream is more appropriate here because reading a stream changes its internal state                
                 */
                let mut reader = BufReader::new(&mut stream);
                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let parts : Vec<&str> = request_line.trim_end().split_whitespace().collect();
                let path = parts.get(1).unwrap_or(&"");

                let response = 
                if *path == "/" {
                    "HTTP/1.1 200 OK\r\n\r\n".to_string()
                }
                else if path.starts_with("/echo") {
                    let content = &path[6..];
                    let content_len = content.len();
                    format!("HTTP/1.1 200 OK\r\n Content-Type: text/plain\r\n Content-Length: {}\r\n  \r\n {}", content_len, content)
                }
                else {
                    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                };

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
