use axum::{Json, Router, extract::State, routing::post};
use core_crate::{AppResult, error::AppError};
use domain::{
    authentication::{request::RequestAuthenticate, response::ResponseAuthenticate},
    user::{ User, request::RequestCreateUser, response::ResposeCreateUser},
};
use infrastructure::{
    middleware::{UserDMC, create, find_by_field},
    utils::{create_jwt_token, hash_password, verify_password},
};
use sqlx::PgPool;

pub fn login() -> Router<PgPool> {
    pub async fn login_user(
        State(db): State<PgPool>,
        Json(req): Json<RequestAuthenticate>,
    ) -> AppResult<Json<ResponseAuthenticate>> {
        let (username, password) = (req.username.as_str(), req.password);
        let user = find_by_field::<UserDMC, User, &str>(db, "username", username)
            .await?
            .ok_or(AppError::Unauthorized)?;
        //verify password
        let ok_password = verify_password(password.as_str(), &user.password_hash).unwrap_or(false);
        if !ok_password {
            return Err(AppError::Unauthorized);
        }
        let token = create_jwt_token(user.pk_user_id);
        Ok(Json(ResponseAuthenticate { token }))
    }
    Router::new().route("/login", post(login_user))
}
pub fn register() -> Router<PgPool> {
    pub async fn register_user(
        State(db): State<PgPool>,
        Json(req): Json<RequestCreateUser>,
    ) -> AppResult<Json<ResposeCreateUser>> {

        let mut entity: domain::user::User = req.into();
        entity.password_hash=hash_password(&entity.password_hash).expect("Failed to hash");
        let created_user = create::<UserDMC, User>(db, entity).await?;
        Ok(Json(ResposeCreateUser::from(created_user)))
    }
    Router::new().route("/register", post(register_user))
}
