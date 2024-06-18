use std::{sync::Arc, u32};

use axum::{
    extract::{Path, Query, State}, routing::get, Json, Router
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct QAListRequest {
    mode: String,
    t: String,
}

#[derive(Serialize)]
struct ServiceList {
    name: String,
    table_name: String,
}

#[derive(Serialize)]
struct QAInfo {
    id: String,
    query: String,
    answer: String,
    date: String,
    vote: String,
    uuid: String,
    rate: String,
    rate_reason: String,
}
#[derive(Serialize)]
struct QAListResponse {
    code: u32,
    message: String,
    qa_list: Vec<QAInfo>
}

mod dao;

#[tokio::main]
async fn main() {

    dao::get_service_list().await;

    let app = Router::new()
        .route("/", get(|| async { "Hello world "}))
        .route("/api/service", get(service_list))
        .route("/api/qa/:service_name", get(get_qalist))
        ;

    let sock = "0.0.0.0:8008";

    println!("BIND={}", sock);

    let listener = tokio::net::TcpListener::bind(sock).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}



async fn service_list() -> Json<ServiceList> {
    let services = ServiceList{
        name: String::from("kaai_poc"),
        table_name: String::from("attention_kaai_poc"),
    };

    Json(services)
}

async fn get_qalist(Path(service_name): Path<String>, _mode:Option<Query<QAListRequest>>) -> Json<QAListResponse> {
    println!("get_qalist={}", service_name);

    if _mode.is_none() {
        println!("get_qalist= mode is none");
    } else {
        let mode = _mode.unwrap();
        println!("get_qalist= mode is {}", mode.mode);
        println!("get_qalist= mode is {}", mode.t);
    }


    let qalist = vec![
        QAInfo{
        id: String::from("id"),
        query: String::from("query"),
        answer: String::from("answer"),
        date: String::from("date"),
        vote: String::from("vote"),
        uuid: String::from("uuid"),
        rate: String::from("rate"),
        rate_reason: String::from("rate_reason"),
    },
    ];

    let qalist_response = QAListResponse {
        code: 0,
        message: String::from("ok"),
        qa_list: qalist
    };

    Json(qalist_response)
}