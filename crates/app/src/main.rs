use api::{auth_routes, user_routes};
use axum::{
    Json, Router,
    extract::Path,
    middleware::{self},
};
use core_crate::{AppResult, config::AppConfig, error::AppError};
use dotenv::dotenv;
use infrastructure::initialize_db;
use infrastructure::middleware::{map_response::mw_map_response, mw_auth::mw_auth};
use serde_json::json;
use tracing::info;
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = AppConfig::from_env().expect("Cann't get env");
    let pool = initialize_db(&cfg.postgres.dsn, cfg.postgres.max_conn).await;

    let app = Router::new()
        // .route("/{msg}", get(say_hello)) // auth
        // .route("/user/{id}", get(get_user))
        .merge(user_routes()).merge(auth_routes())
        .layer(middleware::map_response(mw_map_response)) // 1
        .layer(middleware::from_fn_with_state(pool.clone(), mw_auth)) // 2
        .fallback(|| async { Err::<(), AppError>(AppError::NotFound) })
        .with_state(pool);
    info!("Connect Database successfully");

    info!("Server is running on port: {}", cfg.web.addr);
    let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn say_hello(Path(msg): Path<String>) -> AppResult<Json<serde_json::Value>> {
    info!("->> Function Say Hello");
    if msg.is_empty() {
        Err(AppError::NotFound)
    } else {
        Ok(Json(json!({"msg" : msg})))
    }
}

// #[derive(Serialize, FromRow)]
// pub struct User {
//     pub pk_user_id: i32,
//     pub username: String,
// }

// #[derive(Deserialize)]
// pub struct UserId {
//     pub id: i64,
// }

// pub async fn get_user(State(db): State<PgPool>, Path(id): Path<UserId>) -> AppResult<Json<User>> {
//     let user: User = sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_user" WHERE pk_user_id = $1"#)
//         .bind(id.id)
//         .fetch_optional(&db)
//         .await?
//         .ok_or(AppError::NotFound)?;
//     Ok(Json(user))
// }

// pub async fn mw_map_response(uri: Uri, req_method: Method, res: Response) -> Response {
//     let uuid = Uuid::new_v4();
//     info!("->> MAP RESPONSE");
//     info!("->> UUid: {}", uuid.to_string());
//     info!("->> Method: {}", req_method.to_string());
//     info!("->> Uri: {}", uri.to_string());
//     (StatusCode::ACCEPTED, res).into_response()
// }

// pub async fn mw_auth(req: Request, next: Next) -> AppResult<Response> {
//     info!("->> MIDDLEWARE AUTH");
//     Ok(next.run(req).await)
// }
