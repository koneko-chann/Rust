use std::default;

use crate::{todo::Todo, user::User};
use modql::field::Fields;
use o2o::o2o;
use serde::Deserialize;

#[derive(Deserialize, Fields)]
pub struct RequestGetTodo {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Fields, o2o)]
#[owned_into(Todo)]
#[ghosts(pk_todo_item_id: None, created_at: None, updated_at: None, fk_user_id: i64::MIN)]
pub struct RequestCreateTodo {
    #[map(title)]
    pub title: String,
    #[map(is_completed)]
    pub completed: bool,
    #[map(description)]
    pub description: Option<String>,
}
