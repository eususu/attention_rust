use axum::{
    routing::{get},
    Router,
};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(|| async { "Hello world "}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8008").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
