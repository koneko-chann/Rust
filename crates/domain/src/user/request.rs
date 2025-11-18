use crate::user::User;
use modql::field::Fields;
use o2o::o2o;
use serde::Deserialize;

#[derive(Deserialize, Fields)]
pub struct RequestGetUser {
    pub id: i64,
}

#[derive(Deserialize, Fields, o2o)]
#[owned_into(User)]
#[ghosts(pk_user_id: None)]
pub struct RequestCreateUser {
    #[map(username)] // map sang field username bên User
    pub username: String,
    #[map(password_hash)] // map sang field password_hash bên User
    pub password: String,
}
#[derive(Deserialize, Fields, o2o)]
#[owned_into(User)]
pub struct RequestUpdateUser {
    #[into(pk_user_id)]
    pub id: Option<i64>,
    #[into(username)]
    pub username: String,
    #[into(password_hash)]
    pub password: String,
}
