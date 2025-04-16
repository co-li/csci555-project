/*
Rocket Rust Web Server

Assets:
*.jpg
*.mp4
Copyright © 2025 University of Southern California
*/

#[macro_use] extern crate rocket;
use rocket::http::{Status, ContentType};
use rocket::tokio::time::{sleep, Duration};
use rocket::{fairing::{AdHoc, self}, Rocket, Build, fs::{FileServer, relative}};
use rocket::form::Form;

#[derive(FromForm)]
struct HelloFormData {
    delay: u64,
    message: String,
}


#[get("/")]
async fn index() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::HTML, 
        "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"utf-8\">
            <title>Rust Web Server</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <h2>Rust Web Server</h2>
            <p> /delayed: delay response for 10 seconds</p>
            <p> /plaintext: plain text response</p>
            <p> /json: json message response</p>
            <p> /img: html with image</p>
            <p> /imgs: html with multiple images</p>
            <p> /vid: html with video</p>
            <p> /helloform: POST with form data delay={SECOND}&message=Hello, world! </p>
        </body>
        </html>"
    ))
}

#[get("/delayed")]
async fn delayed() -> (Status, (ContentType, &'static str)) {
    sleep(Duration::from_secs(10)).await;
    (Status::Ok, (ContentType::Text, "Hello, world!"))
}

#[get("/plaintext")]
async fn plaintext() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::Text, "Hello, world!"))
}

#[get("/json")]
async fn json() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::JSON, "{ \"message\": \"Hello, world!\" }"))
}

#[get("/img")]
async fn img() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::HTML, 
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
    ))
}

#[get("/imgs")]
async fn imgs() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::HTML, 
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
    ))
}

#[get("/vid")]
async fn vid() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::HTML, 
        "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"utf-8\">
            <title>Rust Web Server</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <h2>Rust Web Server</h2>
             <video autoplay muted>
                <source src=\"0.mp4\" type=\"video/mp4\">
                Your browser does not support the video.
            </video> 
        </body>
        </html>"
    ))
}

#[post("/helloform", data = "<data>")]
async fn helloform(data: Form<HelloFormData>) -> (Status, (ContentType, &'static str)) {
    sleep(Duration::from_secs(data.delay)).await;
    match data.message.as_str() {
        "Hello, world!" => (Status::Ok, (ContentType::Text, "Hello received")),
        _ => (Status::BadRequest, (ContentType::Text, "Incorrect message received."))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .configure(rocket::Config::figment().merge(("port", 9797)))
        .mount("/", routes![index, delayed, plaintext, json, img, imgs, vid, helloform])
        .mount("/", FileServer::from(relative!("/assets")))
}
