pub mod request;
pub mod response;
use chrono::Utc;
use chrono::{DateTime, NaiveDateTime};
use modql::field::Fields;
use sqlx::prelude::FromRow;

#[derive(Fields, FromRow, Debug, Clone)]
pub struct Todo {
    pub pk_todo_item_id: Option<i32>,
    pub fk_user_id: i64,
    pub title: String,
    pub is_completed: bool,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
