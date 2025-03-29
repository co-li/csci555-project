#[macro_use] extern crate rocket;
use rocket::http::{Status, ContentType};
use rocket::tokio::time::{sleep, Duration};

#[get("/")]
async fn index() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::HTML, 
        "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"utf-8\">
            <title>Rocket Rust Web Server</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <h2>Rocket Rust Web Server</h2>
            <p> /delayed: delay response for 10 seconds</p>
            <p> /plaintext: plain text response</p>
            <p> /json: json message response</p>
        </body>
        </html>"
    ))
}

#[get("/delayed")]
async fn delayed() -> String {
    sleep(Duration::from_secs(10)).await;
    String::from("Hello, world!")
}

#[get("/plaintext")]
async fn plaintext() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::Text, "Hello, world!"))
}

#[get("/json")]
async fn json() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::JSON, "{ \"message\": \"Hello, world!\" }"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, delayed, plaintext, json])
}
