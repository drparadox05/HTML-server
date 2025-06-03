#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use std::thread;


/*
- Header names are case-insensitive.
- The two headers (content type & content length) are required for the client to be able to parse the response body. 
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
                thread::spawn(move || {
                    let mut reader = BufReader::new(&mut stream);
                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let mut user_agent = String::new();
                    let mut header_line = String::new();
                    
                    let parts : Vec<&str> = request_line.trim_end().split_whitespace().collect();
                    let path = parts.get(1).unwrap_or(&"");
                    
                    loop {
                        header_line.clear();
                        let bytes_read = reader.read_line(&mut header_line).unwrap();
                        if (header_line == "\r\n" || bytes_read == 0){
                            break;
                        }
                        if let Some(user_agent_stripped) = header_line.strip_prefix("User-Agent: "){
                            user_agent = user_agent_stripped.trim_end().to_string();
                        }
                    }

                    let response = 
                    if *path == "/" {
                        "HTTP/1.1 200 OK\r\n\r\n".to_string()
                    }
                    else if path.starts_with("/echo") {
                        let content = &path[6..];
                        let content_len = content.len();
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content_len, content)
                    }
                    else if *path == "/user-agent" {
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent)
                    }
                    else {
                        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                    };

                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Failed to write to stream: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
