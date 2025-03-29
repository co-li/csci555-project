use axum::{
    routing::get,
    response::IntoResponse,
    http::{HeaderMap, header},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/plaintext", get(plain_text))
        .route("/json", get(json));

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn plain_text() -> &'static str {
    "Hello, World!"
}

async fn json() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    (headers, "{\"message\":\"Hello, World!\"}")
}
