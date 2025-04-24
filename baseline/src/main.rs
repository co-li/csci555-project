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
    let mut buf_reader = BufReader::new(&stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();

    if request_line.starts_with("GET") {
        let path = if let Some(stripped) = request_line.strip_prefix("GET /") {
            stripped.strip_suffix(" HTTP/1.1\r\n")
        } else {
            None
        };
        let (status_line, contents, content_type) = match path {
            Some("") => ("HTTP/1.1 200 OK", fs::read_to_string("src/baseline.html").unwrap(), "text/html"),
            Some("delayed") => {
                thread::sleep(Duration::from_secs(10));
                ("HTTP/1.1 200 OK", fs::read_to_string("src/baseline.html").unwrap(), "text/html")
            },
            Some("plaintext") => ("HTTP/1.1 200 OK", String::from("Hello, world!"), "text/plain"),
            Some("json") => ("HTTP/1.1 200 OK", json!({"message":  "Hello, world!"}).to_string(), "application/json"),
            Some("img") => ("HTTP/1.1 200 OK", fs::read_to_string("src/img.html").unwrap(), "text/html"),
            Some("imgs") => ("HTTP/1.1 200 OK", fs::read_to_string("src/imgs.html").unwrap(), "text/html"),
            Some("vid") => ("HTTP/1.1 200 OK", fs::read_to_string("src/vid.html").unwrap(), "text/html"),
            Some(file_name) if file_name.ends_with(".jpg") => ("HTTP/1.1 200 OK", format!("assets/{}", file_name), "image/jpg"),
            Some(file_name) if file_name.ends_with(".mp4") => ("HTTP/1.1 200 OK", format!("assets/{}", file_name), "video/mp4"),
            _ => ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("src/404.html").unwrap(), "text/html"),
        };
    
        if content_type == "image/jpg" {
            let content_byte = fs::read(contents).unwrap();
            let length = content_byte.len();
            let response = 
                format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n");
            stream.write(response.as_bytes()).unwrap();
            stream.write(&content_byte).unwrap();
        } else if content_type == "video/mp4" {
            let content_byte = fs::read(contents).unwrap();
            let length = content_byte.len();
            let response = 
                format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n");
            stream.write(response.as_bytes()).unwrap();
            stream.write(&content_byte).unwrap();
        } else {
            let length = contents.len();
            let response =
                format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    } else if request_line.starts_with("POST") {
        let path = if let Some(stripped) = request_line.strip_prefix("POST /") {
            stripped.strip_suffix(" HTTP/1.1\r\n")
        } else {
            None
        };

        let (status_line, contents, content_type) = match path {
            Some("helloform") => {
                let mut form_length = 0;
                loop {
                    let mut request_line = String::new();
                    buf_reader.read_line(&mut request_line).unwrap();
                    if request_line == "\r\n" {
                        break;
                    } 
                    if let Some(len) = request_line.strip_prefix("Content-Length: ") {  
                        form_length = len.trim().parse::<usize>().unwrap();
                    }
                }
                let mut form_data_vec = vec![0; form_length];
                buf_reader.read_exact(&mut form_data_vec).unwrap();
                let form_data = String::from_utf8_lossy(&form_data_vec);
                let mut delay_sec = 0;
                if let Some(delay) = form_data.strip_suffix("&message=Hello, world!") {  
                    if let Some(delay_data) = delay.strip_prefix("delay=") {
                        delay_sec = delay_data.trim().parse::<u64>().unwrap();
                    }
                }
                thread::sleep(Duration::from_secs(delay_sec));
                ("HTTP/1.1 200 OK", String::from("Hello received"), "text/plain")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("src/404.html").unwrap(), "text/html"),
        };

        let length = contents.len();
            let response =
                format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
