use axum::Router;
use infrastructure::middleware::{
    self, UserDMC,
    mw_auth::{self, mw_auth_with_jwt},
};
use sqlx::PgPool;

use crate::{
    authentication::{login, register},
    user::{create_user_route, get_user_route, get_users_route},
};
pub mod authentication;
pub mod todo;
pub mod user;
pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .merge(get_user_route())
                .merge(get_users_route())
                .merge(create_user_route::<UserDMC>())
                .merge(user::update_user_route())
                // .merge(user::delete_user_route()),
        )
        .layer(axum::middleware::from_fn(mw_auth_with_jwt))
    //Not found route can add he
}
pub fn auth_routes() -> Router<PgPool> {
    Router::new().nest("/api/v1", Router::new().merge(login()).merge(register()))
}
pub fn todo_routes() -> Router<PgPool> {
    Router::new()
        .nest("/api/v1", Router::new().merge(todo::todo_routes()))
        .layer(axum::middleware::from_fn(mw_auth_with_jwt))
    //Not found route can add he
}
