use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseAuthenticate {
    pub token: String,
}

