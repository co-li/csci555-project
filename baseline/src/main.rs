/*
Baseline Rust Web Server

Reference:
S. Klabnik and C. Nichols, The Rust Programming Language, 2nd Edition, 2nd Edition. New York: No Starch Press, 2023.
*/

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use baseline::ThreadPool;
use serde_json::json;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents, content_type) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", fs::read_to_string("src/baseline.html").unwrap(), "text/html"),
        "GET /delayed HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", fs::read_to_string("src/baseline.html").unwrap(), "text/html")
        }
        "GET /plaintext HTTP/1.1" => ("HTTP/1.1 200 OK", String::from("Hello, world!"), "text/plain"),
        "GET /json HTTP/1.1" => ("HTTP/1.1 200 OK", json!({"message":  "Hello, world!"}).to_string(), "application/json"),
        _ => ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("src/404.html").unwrap(), "text/html"),
    };
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
