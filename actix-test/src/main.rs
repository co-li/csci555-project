// #[global_allocator]
// static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use std::io;
use std::time::Duration;

use actix_http::{HttpService, KeepAlive};
use actix_service::map_config;
use actix_web::{
    dev::{AppConfig, Server},
    http::{
        header::{HeaderValue, CONTENT_TYPE, SERVER},
        StatusCode,
    },
    web::{self, Bytes, BytesMut},
    App, HttpResponse,
};
use simd_json_derive::Serialize;
use bytes::BufMut;

pub const JSON_MSG_SIZE: usize = 27;

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
    println!("JSON response: {:?}", res);
    res
}

async fn plaintext() -> HttpResponse<Bytes> {
    let mut res = HttpResponse::with_body(StatusCode::OK, Bytes::from_static(b"Hello, World!"));
    res.headers_mut()
        .insert(SERVER, HeaderValue::from_static("A"));
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    println!("Plaintext response: {:?}", res);
    res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started HTTP server: 127.0.0.1:8080");

    // start http server
    Server::build()
        .backlog(1024)
        .bind("tfb-actix-web", "0.0.0.0:8080", || {
            HttpService::build()
                .keep_alive(KeepAlive::Os)
                .client_request_timeout(Duration::ZERO)
                .h1(map_config(
                    App::new()
                        .service(web::resource("/json").to(json))
                        .service(web::resource("/plaintext").to(plaintext)),
                    |_| AppConfig::default(),
                ))
                .tcp()
        })?
        .run()
        .await
}
