use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use core_crate::AppResult;
use domain::{
    todo::{self, Todo, request::RequestDeleteTodo, response::ResponseGetTodo},
    user,
};
use infrastructure::middleware::{TodoDMC, find_by_field, mw_auth::AuthUser};
use sqlx::PgPool;
use tracing::info;

pub fn get_todo() -> Router<PgPool> {
    pub async fn get_todo_by_auth_user(
        State(db): State<PgPool>,
        Extension(auth_user): Extension<AuthUser>,
    ) -> AppResult<(StatusCode, Json<Vec<ResponseGetTodo>>)> {
        let user_id = auth_user.user_id;
        let todos = find_by_field::<TodoDMC, Todo, i64>(db, "fk_user_id", user_id).await?;
        let response: Vec<ResponseGetTodo> = match todos {
            Some(todos) => todos.into_iter().map(ResponseGetTodo::from).collect(),
            None => vec![],
        };
        info!(
            "->> Retrieved {} todo items for user_id {}",
            response.len(),
            user_id
        );
        Ok((StatusCode::OK, Json(response)))
    }
    Router::new().route("/todo", axum::routing::get(get_todo_by_auth_user))
}
pub fn get_todo_by_id() -> Router<PgPool> {
    pub async fn get_todo_by_id_for_auth_user(
        State(db): State<PgPool>,
        Extension(auth_user): Extension<AuthUser>,
        Path(todo_id): Path<i32>,
    ) -> AppResult<(StatusCode, Json<domain::todo::response::ResponseGetTodo>)> {
        info!("->> Fetching todo item with id {:?} for user_id {}", todo_id, auth_user.user_id);
        let user_id = auth_user.user_id;
        let todo_opt = find_by_field::<TodoDMC, Todo, i64>(db, "pk_todo_item_id", todo_id as i64).await?;
        let todo = todo_opt
            .ok_or(core_crate::error::AppError::NotFound)?
            .into_iter()
            .find(|t| t.fk_user_id == user_id)
            .ok_or(core_crate::error::AppError::Forbidden)?;
        let response = domain::todo::response::ResponseGetTodo::from(todo);
        info!(
            "->> Retrieved todo item with id {:?} for user_id {}",
            response.id, user_id
        );
        Ok((StatusCode::OK, Json(response)))
    }
    Router::new().route("/todo/{todo_id}", axum::routing::get(get_todo_by_id_for_auth_user))
}
pub fn create_todo() -> Router<PgPool> {
    pub async fn create_todo_for_auth_user(
        State(db): State<PgPool>,
        Extension(auth_user): Extension<AuthUser>,
        Json(req): Json<domain::todo::request::RequestCreateTodo>,
    ) -> AppResult<(StatusCode, Json<domain::todo::response::ResponseCreateTodo>)> {
        let mut entity: domain::todo::Todo = req.into();
        entity.fk_user_id = auth_user.user_id;
        let created_todo = infrastructure::middleware::create::<TodoDMC, Todo>(db, entity).await?;
        let response = domain::todo::response::ResponseCreateTodo::from(created_todo);
        info!(
            "Created todo item with id {:?} for user_id {}",
            response.id, auth_user.user_id
        );
        Ok((StatusCode::CREATED, Json(response)))
    }
    Router::new().route("/todo", axum::routing::post(create_todo_for_auth_user))
}
pub fn update_todo() -> Router<PgPool> {
    pub async fn update_todo_for_auth_user(
        State(db): State<PgPool>,
        Extension(auth_user): Extension<AuthUser>,
        Json(req): Json<domain::todo::request::RequestUpdateTodo>,
    ) -> AppResult<Response> {
        let mut entity: domain::todo::Todo = req.into();
        entity.fk_user_id = auth_user.user_id;
        infrastructure::middleware::update::<TodoDMC, Todo>(db, entity).await?;
        Ok(StatusCode::OK.into_response())
    }
    Router::new().route("/todo", axum::routing::put(update_todo_for_auth_user))
}
pub fn delete_todo() -> Router<PgPool> {
    pub async fn delete_todo_for_auth_user(
        State(db): State<PgPool>,
        Extension(auth_user): Extension<AuthUser>,
        Path(todo_id): Path<i32>,
    ) -> AppResult<Response> {
        let user_id = auth_user.user_id;
        let todo_opt = find_by_field::<TodoDMC, Todo, i64>(db.clone(), "pk_todo_item_id", todo_id as i64).await?;
        let todo = todo_opt
            .ok_or(core_crate::error::AppError::NotFound)?
            .into_iter()
            .find(|t| t.fk_user_id == user_id)
            .ok_or(core_crate::error::AppError::Forbidden)?;
        let todo_id = todo.pk_todo_item_id.unwrap_or_default();
        if todo_id == 0 {
            return Err(core_crate::error::AppError::NotFound);
        }
        infrastructure::middleware::delete::<TodoDMC, Todo>(db, RequestDeleteTodo { id: Some(todo_id) }.into()).await?;
        Ok(StatusCode::OK.into_response())
    }
    Router::new().route("/todo/{todo_id}", axum::routing::delete(delete_todo_for_auth_user))
}
pub fn todo_routes() -> Router<PgPool> {
    get_todo().merge(create_todo()).merge(update_todo()).merge(delete_todo()).merge(get_todo_by_id())
}
