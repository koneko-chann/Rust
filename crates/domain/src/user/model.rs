use serde::Deserialize;
#[derive(Deserialize)]
pub struct UserModel {
    pub pk_user_id: i64,
    pub username: String,
}
