use std::time::Duration;
use axum::{
    http::{header, StatusCode}, response::IntoResponse, routing::{get, post}, Form, Router
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct FormData {
    delay: u64,
    message: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/plaintext", get(plain_text))
        .route("/json", get(json))
        .route("/delayed", get(delayed))
        .route("/helloform", post(hello_form))
        .nest_service("/assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn plain_text() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        "Hello, World!",
    )

}

async fn json() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        "{\"message\":\"Hello, World!\"}",
    )
}

async fn delayed() -> impl IntoResponse {
    tokio::time::sleep(Duration::from_secs(10)).await;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        "Hello, World!",
    )
}

async fn hello_form(Form(data): Form<FormData>) -> impl IntoResponse {
    tokio::time::sleep(Duration::from_secs(data.delay)).await;
    match data.message.as_str() {
        "Hello, world!" => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain")],
            "Hello received",
        ),
        _ => (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "text/plain")],
            "Incorrect message received",
        ),
    }
}
