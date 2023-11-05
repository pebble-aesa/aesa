mod types;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing::post,
  Json, Router,
};
use sqlx::{PgPool, QueryBuilder};
use tower_http::cors::CorsLayer;
use types::{GetMessage, InsertMessage};

#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres(local_uri = "postgres://aesa:aesa@localhost:5432/aesa")]
  pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
  sqlx::migrate!()
    .run(&pool)
    .await
    .expect("Failed to run migrations");

  let router: Router = Router::new()
    .route("/messages/new", post(new))
    .route("/messages/all", post(all))
    .route("/messages/:id", post(id))
    .with_state(pool)
    .layer(CorsLayer::very_permissive());

  Ok(router.into())
}

async fn new(
  State(pool): State<PgPool>,
  Json(content): Json<InsertMessage>,
) -> Result<StatusCode, StatusCode> {
  let mut query = QueryBuilder::new("INSERT INTO messages (title, nickname, content) VALUES (");
  let query = query
    .push_bind(content.title)
    .push(", ")
    .push_bind(content.nickname)
    .push(", ")
    .push_bind(content.content)
    .push(") RETURNING id")
    .build_query_as::<GetMessage>();

  let x = query.fetch_one(&pool).await;
  if let Ok(_) = x {
    Ok(StatusCode::CREATED)
  } else {
    println!("{:?}", x);
    Err(StatusCode::INTERNAL_SERVER_ERROR)
  }
}

async fn all(State(pool): State<PgPool>) -> Result<Json<Vec<GetMessage>>, StatusCode> {
  let mut query = QueryBuilder::new("SELECT * FROM messages");
  let query = query.build_query_as::<GetMessage>();

  if let Ok(boards) = query.fetch_all(&pool).await {
    Ok(Json(boards))
  } else {
    Err(StatusCode::INTERNAL_SERVER_ERROR) // TODO: This endpoint always returns 500
  }
}

async fn id(
  State(pool): State<PgPool>,
  Path(id): Path<i32>,
) -> Result<Json<GetMessage>, StatusCode> {
  let mut query = QueryBuilder::new("SELECT * FROM messages WHERE id = ");
  let query = query.push_bind(id).build_query_as::<GetMessage>();

  if let Ok(board) = query.fetch_one(&pool).await {
    Ok(Json(board))
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}