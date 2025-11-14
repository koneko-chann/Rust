use serde::Deserialize;
use modql::field::Fields;
use o2o::o2o;
use crate::user::User;

#[derive(Deserialize,Fields)]
pub struct RequestAuthenticate {
    pub username: String,
    pub password: String,
}
