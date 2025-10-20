use dotenv::var;
use crate::AppResult;

pub fn get_dsn() -> String {
    var("DSN").expect("Dont have DSN env variable")
}
pub fn get_max_conn() -> u32 {
    var("MAX_CONN")
        .unwrap_or_else(|_| "4".to_string())
        .parse::<u32>()
        .expect("MAX_CONN should be a number")
}
pub fn get_port() -> String {
    var("PORT").unwrap_or_else(|_| "0.0.0.0:3000".to_string())
}
#[derive(serde::Deserialize)]
pub struct WebConfig {
    pub addr: String,
}
#[derive(serde::Deserialize)]
pub struct Postgres {
    pub dsn: String,
    pub max_conn: u32,
}
#[derive(serde::Deserialize)]
pub struct AppConfig {
    pub web: WebConfig,
    pub postgres: Postgres,
}
impl AppConfig {
    pub fn from_env() -> AppResult<AppConfig> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(config.try_deserialize()?)
    }
}


