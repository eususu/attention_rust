use axum::http::StatusCode;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};


table! {
  tables(id) {
    id -> Integer,
    table_name -> Text,
  }
}

#[derive(serde::Serialize, Selectable, Queryable, Debug)]
struct Table {
  id: i32,
  table_name: String,
}

pub async fn get_service_list() {

  let db_url = "postgres://postgres:postgres@host/postgres";
  let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
  let pool = bb8::Pool::builder().build(config).await.unwrap();

  let mut _conn = pool.get_owned().await.map_err(internal_error).unwrap();

  let res = tables::table
  .select(Table::as_select())
  .load(&mut _conn)
  .await
  .map_err(internal_error);

  println!("{:?}", res.expect("errrrror"))
  

}

fn internal_error<E>(err: E) -> (StatusCode, String)
where 
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
