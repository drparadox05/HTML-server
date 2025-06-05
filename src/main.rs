#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Read, Write};
use std::thread;
use std::fs;
use std::env;
use std::path::PathBuf;


/*
- Header names are case-insensitive.
- The two headers (content type & content length) are required for the client to be able to parse the response body. 
Note that each header ends in a CRLF, and the entire header section also ends in a CRLF.
*/


fn main() {

    // this is to parse the --directory flags to fetch the filename
    let args : Vec<String> = env::args().collect();
    let mut directory = String::new();
    for i in 0..args.len() {
        if args[i] == "--directory" && i + 1 < args.len() {
            directory = args[i+1].clone();
        }
    }

    // the program listens to the specified port
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            /*
            mutable borrow of stream is more appropriate here because reading a stream changes its internal state
            Also, bufReader is used here which allocates a buffer and lets you read the data in small pieces instead of large chuncks
            reading the data line-by-line or byte-by-byte from TcpStream is slow and inefficient, each call may result in syscall which is expensive

            Imagine you are pouring water from a bucket, now:
            - without buffering: you are taking one drop at a time
            - with buffering: you fill a bucket(buffer) and scoop from it
            */
            Ok(mut stream) => {
                println!("accepted new connection");
                let directory = directory.clone();
                thread::spawn(move || {
                    let mut reader = BufReader::new(&mut stream);
                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let mut user_agent = String::new();
                    let mut header_line = String::new();
                    
                    let parts : Vec<&str> = request_line.trim_end().split_whitespace().collect();
                    let path = parts.get(1).unwrap_or(&"");
                    let method = parts.get(0).unwrap_or(&"");
                    let content_length = 0; 

                    loop {
                        header_line.clear();
                        let bytes_read = reader.read_line(&mut header_line).unwrap();
                        if header_line == "\r\n" || bytes_read == 0 {
                            break;
                        }
                        if let Some(user_agent_stripped) = header_line.strip_prefix("User-Agent: "){
                            user_agent = user_agent_stripped.trim_end().to_string();
                        }
                        if let Some(content_len) = header_line.strip_prefix("Content-Length: "){
                            content_length = content_len.trim().parse::<usize>().unwrap_or(0);
                        }
                    }

                    let response = 
                    if *path == "/" {
                        "HTTP/1.1 200 OK\r\n\r\n".to_string()
                    }
                    else if path.starts_with("/echo/") {
                        let content = &path[6..];
                        let content_len = content.len();
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content_len, content)
                    }
                    else if *path == "/user-agent" {
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent)
                    }
                    else if path.starts_with("/files/") {
                        if *method == "POST" {
                            let mut body = vec![0; content_length];
                            reader.read_exact(&mut body).unwrap();
                            let file_name = &path[7..];
                            let mut file_path = PathBuf::from(&directory);
                            file_path.push(file_name);
                            fs::write(file_path, body).unwrap();
                            "HTTP/1.1 201 Created\r\n\r\n".to_string()
                        }
                        else {
                            let file_name = &path[7..];
                            let mut file_path = PathBuf::from(&directory);
                            file_path.push(file_name);
                            match fs::read_to_string(file_path) {
                                Ok(file_bytes) => 
                                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", file_bytes.len(), file_bytes),
                                Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                            }
                        }
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
