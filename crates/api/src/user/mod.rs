use std::result;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    middleware,
    routing::Route,
};
use core_crate::{AppResult, error::AppError};
use domain::user::request::RequestGetUser;
use domain::user::{User, request::RequestCreateUser, response::ResposeCreateUser};
use infrastructure::middleware::{
    DMC, UserDMC, find_by_field, list,
    mw_auth::{AuthUser, mw_auth_with_jwt, mw_auth_with_secret_key},
};
use sqlx::PgPool;

pub fn get_user_route() -> Router<PgPool> {
    pub async fn get_user_by_id(
        Extension(auth_user): Extension<AuthUser>,
        State(db): State<PgPool>,
        Path(_id): Path<RequestGetUser>,
    ) -> AppResult<Json<User>> {
        if _id.id != auth_user.user_id {
            return Err(AppError::Forbidden);
        }
        let user = find_by_field::<UserDMC, User, i32>(db, "pk_user_id", auth_user.user_id)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(Json(user))
    }
    Router::new().route("/user/{id}", axum::routing::get(get_user_by_id))
}
pub fn get_users_route() -> Router<PgPool> {
    pub async fn get_users(State(db): State<PgPool>) -> AppResult<Json<Vec<User>>> {
        let users = list::<UserDMC, User>(db).await?;
        Ok(users)
    }
    Router::new().route("/users", axum::routing::get(get_users))
}
pub fn update_user_route() -> Router<PgPool> {
    pub async fn update_user(
        State(db): State<PgPool>,
        Json(req): Json<domain::user::request::RequestUpdateUser>,
    ) -> AppResult<()> {
        let entity: domain::user::User = req.into();
        infrastructure::middleware::update::<UserDMC, User>(db, entity).await
    }
    Router::new().route("/user/update", axum::routing::put(update_user))
}
pub fn delete_user_route() -> Router<PgPool> {
    pub async fn delete_user(State(db): State<PgPool>, Path(user_id): Path<i32>) -> AppResult<()> {
        infrastructure::middleware::delete(db, user_id).await
    }
    Router::new().route("/user/delete/{user_id}", axum::routing::delete(delete_user))
}
pub fn create_user_route<MC>() -> Router<PgPool>
where
    MC: DMC + 'static,
{
    pub async fn create_user<LMC>(
        State(db): State<PgPool>,
        Json(rq): Json<domain::user::request::RequestCreateUser>,
    ) -> AppResult<Json<ResposeCreateUser>>
    where
        LMC: DMC,
    {
        
        let entity =
            infrastructure::middleware::create::<LMC, User>(db, rq.into()).await?;
        let parsed: ResposeCreateUser = entity.into();
        Ok(Json(parsed))
    }
    Router::new().route("/user/create", axum::routing::post(create_user::<MC>))
}
