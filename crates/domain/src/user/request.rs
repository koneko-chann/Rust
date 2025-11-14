use modql::field::Fields;
use serde::Deserialize;
use o2o::o2o;
use crate::user::User;

#[derive(Deserialize, Fields)]
pub struct RequestGetUser {
    pub id: i32,
}

#[derive(Deserialize, Fields,o2o)]
#[owned_into(User)]
#[ghosts(pk_user_id: 0)]
pub struct RequestCreateUser {
    #[map(username)]          // map sang field username bên User
    pub username: String,
    #[map(password_hash)]      // map sang field password_hash bên User
    pub password: String,
}
#[derive(Deserialize, Fields, o2o)]
#[owned_into(User)]
pub struct RequestUpdateUser {
    #[into(pk_user_id)]
    pub id: i64,
    #[into(username)]
    pub username: String,
    #[into(password_hash)]
    pub password: String,
}