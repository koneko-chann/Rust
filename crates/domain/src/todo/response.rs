use chrono::NaiveDateTime;
use o2o::o2o;
use serde::Serialize;

use crate::todo::Todo;

#[derive(Serialize, o2o)]
#[from_owned(Todo)]
pub struct ResponseGetTodo {
    #[from(pk_todo_item_id)]
    pub id: Option<i32>,
    pub title: String,
    #[from(is_completed)]
    pub completed: bool,
    pub description: Option<String>,
    #[from(created_at)]
    pub created_at: Option<NaiveDateTime>,
    #[from(updated_at)]
    pub updated_at: Option<NaiveDateTime>,
}

pub type ResponseCreateTodo = ResponseGetTodo;
