// #[global_allocator]
// static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use std::{io, time::Duration};

use actix_files as fs;
use actix_web::{
    get, http::{
        header::{ContentType, HeaderValue, CONTENT_TYPE, SERVER}, StatusCode,
    }, post, rt::time::sleep, web::{self, Bytes, BytesMut}, App, HttpResponse, HttpServer
};
use bytes::BufMut;
use simd_json_derive::Serialize;
use serde::Deserialize;

pub const JSON_MSG_SIZE: usize = 27;

#[derive(Deserialize)]
struct FormData {
    delay: u64,
    message: String,
}

pub struct Writer<'a, B>(pub &'a mut B);

impl<'a, B: BufMut> io::Write for Writer<'a, B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.put_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Serialize)]
pub struct Message {
    pub message: &'static str,
}

#[get("/json")]
async fn json() -> HttpResponse<Bytes> {
    let message = Message {
        message: "Hello, World!",
    };

    let mut body = BytesMut::with_capacity(JSON_MSG_SIZE);
    message.json_write(&mut Writer(&mut body)).unwrap();

    let mut res = HttpResponse::with_body(StatusCode::OK, body.freeze());
    res.headers_mut()
        .insert(SERVER, HeaderValue::from_static("A"));
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // println!("JSON response: {:?}", res);
    res
}

#[get("/plaintext")]
async fn plaintext() -> HttpResponse<Bytes> {
    let mut res = HttpResponse::with_body(StatusCode::OK, Bytes::from_static(b"Hello, World!"));
    res.headers_mut()
        .insert(SERVER, HeaderValue::from_static("A"));
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    // println!("Plaintext response: {:?}", res);
    res
}

/*
#[get("/img")]
async fn img() -> HttpResponse {
    HttpResponse::Ok().insert_header(ContentType::html()).body(
        "<!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset=\"utf-8\">
                <title>Rust Web Server</title>
            </head>
            <body>
                <h1>Hello!</h1>
                <h2>Rust Web Server</h2>
                <img src=\"0.jpg\" alt=\"example img\" style=\"float:left\">
            </body>
            </html>"
    )
}

#[get("/imgs")]
async fn imgs() -> HttpResponse {
    HttpResponse::Ok().insert_header(ContentType::html()).body(
        "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"utf-8\">
            <title>Rust Web Server</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <h2>Rust Web Server</h2>
            <img src=\"0.jpg\" alt=\"example img\" style=\"float:left\">
            <img src=\"1.jpg\" alt=\"example img\" style=\"float:left\">
            <img src=\"2.jpg\" alt=\"example img\" style=\"float:left\">
            <img src=\"3.jpg\" alt=\"example img\" style=\"float:left\">
            <img src=\"4.jpg\" alt=\"example img\" style=\"float:left\">
            <img src=\"5.jpg\" alt=\"example img\" style=\"float:left\">
            <img src=\"6.jpg\" alt=\"example img\" style=\"float:left\">
            <img src=\"7.jpg\" alt=\"example img\" style=\"float:left\">
        </body>
        </html>"
    )
}

#[get("/vid")]
async fn vid() -> HttpResponse {
    HttpResponse::Ok().insert_header(ContentType::html()).body(
        "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"utf-8\">
            <title>Rust Web Server</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <h2>Rust Web Server</h2>
            <video width=\"320\" height=\"240\" controls>
                <source src=\"0.mp4\" type=\"video/mp4\">
                Your browser does not support the video tag.
            </video>
        </body>
        </html>"
    )
}
*/

#[post("/helloform")]
async fn helloform(form: web::Form<FormData>) -> HttpResponse {
    let delay = form.delay;
    let message = &form.message;

    // print!("Delay: {}, Message: {}\n", delay, message);
    sleep(Duration::from_secs(delay)).await;
    match message.as_str() {
        "Hello, world!" => {
            HttpResponse::Ok()
                .insert_header(ContentType::plaintext())
                .body(
                    "Hello received."
                )
        }
        _ => {
           HttpResponse::BadRequest()
                .insert_header(ContentType::html())
                .body(
                    "Incorrect message received."
                )
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(json)
            .service(plaintext)
            // .service(img)
            // .service(imgs)
            .service(helloform)
            // .service(vid)
            .service(
                fs::Files::new("/", "assets")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
