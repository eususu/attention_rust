use axum::{
    routing::get, Router,
    Json
};
use serde::Serialize;

#[derive(Serialize)]
struct ServiceList {
    name: String,
    table_name: String,
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(|| async { "Hello world "}))
        .route("/api/service", get(service_list))
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8008").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}



async fn service_list() -> Json<ServiceList> {
    let services = ServiceList{
        name: String::from("kaai_poc"),
        table_name: String::from("attention_kaai_poc"),
    };

    Json(services)
}
