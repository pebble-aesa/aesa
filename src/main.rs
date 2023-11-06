mod types;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing::post,
  Json, Router,
};
use sqlx::{PgPool, QueryBuilder};
use tower_http::{
  cors::CorsLayer,
  services::{ServeDir, ServeFile},
};
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
    .nest_service(
      "/",
      ServeDir::new("public/dist")
        .fallback(ServeFile::new("public/dist/404.html"))
        .append_index_html_on_directories(true),
    )
    .route("/api/new", post(new)) // TODO: This endpoint always returns 500
    .route("/api/all", post(all))
    .route("/api/:id", post(id))
    .route("/api/upvote/:id", post(upvote)) // TODO: This endpoint always returns 404
    .route("/api/downvote/:id", post(downvote)) // TODO: This endpoint always returns 404
    .with_state(pool)
    .layer(CorsLayer::permissive());

  Ok(router.into())
}

async fn new(
  State(pool): State<PgPool>,
  Json(content): Json<InsertMessage>,
) -> Result<StatusCode, StatusCode> {
  let mut query =
    QueryBuilder::new("INSERT INTO messages (title, nickname, content, score) VALUES (");
  let query = query
    .push_bind(content.title)
    .push(", ")
    .push_bind(content.nickname)
    .push(", ")
    .push_bind(content.content)
    .push(", 0) RETURNING id")
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
    Err(StatusCode::INTERNAL_SERVER_ERROR)
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

async fn upvote(
  State(pool): State<PgPool>,
  Path(id): Path<i32>,
) -> Result<Json<GetMessage>, StatusCode> {
  let mut query = QueryBuilder::new("UPDATE messages SET score = score + 1 WHERE id = ");
  let query = query.push_bind(id).build_query_as::<GetMessage>();

  if let Ok(board) = query.fetch_one(&pool).await {
    Ok(Json(board))
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}

async fn downvote(
  State(pool): State<PgPool>,
  Path(id): Path<i32>,
) -> Result<Json<GetMessage>, StatusCode> {
  let mut query = QueryBuilder::new("UPDATE messages SET score = score - 1 WHERE id = ");
  let query = query.push_bind(id).build_query_as::<GetMessage>();

  if let Ok(board) = query.fetch_one(&pool).await {
    Ok(Json(board))
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}
