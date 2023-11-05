use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InsertMessage {
  pub title: String,
  pub nickname: String,
  pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GetMessage {
  pub id: i32,
  pub title: String,
  pub nickname: String,
  pub content: String,
}
