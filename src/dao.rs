use axum::{http::StatusCode, Json};

use crate::ServiceList;


pub async fn get_service_list() -> Json<Vec<ServiceList>> {


  let response:Vec<ServiceList> = vec![
    ServiceList {
      name: String::from("kaai-poc"),
      table_name: String::from("attention_kaai_poc")
    },
    ServiceList {
      name: String::from("sentimental-test"),
      table_name: String::from("attention_sentimental-test")
    },

  ];


  Json::from(response)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where 
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
