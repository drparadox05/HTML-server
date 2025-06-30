# HTTP Server (Rust)

This project implements a basic HTTP/1.1 server in Rust. The server is designed to demonstrate low-level HTTP protocol handling, multithreaded request processing, and file I/O.

## Technical Overview

### Features
- **Multithreading:**
- **Buffered I/O:**
- **Gzip Compression:**
- **File Operations:**
- **Custom Endpoints:**
- **Error Handling:**
- **Connection Handling:**

### Usage

#### Build
```sh
cargo build --release
```

#### Run
```sh
cargo run -- --directory <directory>
```

#### Example Requests
- `GET /` → 200 OK
- `GET /echo/hello` → 200 OK, body: `hello` (gzip if requested)
- `GET /user-agent` → 200 OK, body: user agent string
- `GET /files/foo.txt` → 200 OK, body: file contents
- `POST /files/foo.txt` (with body) → 201 Created, writes file

