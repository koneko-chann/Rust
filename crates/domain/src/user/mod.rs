use modql::field::Fields;
use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod model;
pub mod request;
pub mod response;

#[derive(Serialize, FromRow, Fields, Debug, Clone)]
pub struct User {
    pub pk_user_id: Option<i64>,
    pub username: String,
    pub password_hash: String,
}
pub trait HasPrimary {
    const PRIMARY_NAME: &'static str;
    fn primary_value(&self) -> sea_query::Value; // đặt tên khác để khỏi nhầm với &ref
}

impl HasPrimary for User {
    const PRIMARY_NAME: &'static str = "pk_user_id";
    fn primary_value(&self) -> sea_query::Value {
        sea_query::Value::Int(self.pk_user_id.map(|id| id as i32))
    }
}
#[derive(Serialize, FromRow, Fields)]
pub struct Course {
    pub pk_course_id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, FromRow, Fields)]
pub struct RequestCreateCourse {
    pub pk_course_id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, FromRow, Fields)]
pub struct ResponseCreateCourse {
    pub pk_course_id: i64,
    pub title: String,
}
