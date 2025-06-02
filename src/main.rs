#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
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
