use o2o::o2o;
use serde::Serialize;

#[derive(Serialize,o2o)]
#[from_owned(crate::user::User)]
pub struct ResposeCreateUser {
    #[from(pk_user_id)]
    pub id: i64,
    #[from(username)]
    pub username: String,
}
