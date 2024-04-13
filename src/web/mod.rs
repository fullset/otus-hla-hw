use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    extract::Json,
    routing::{get, post},
    Extension, Router, Server,
};
use serde::{Deserialize, Serialize};
use sha256::digest;

use uuid::Uuid;

mod dialog;
mod user;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub id: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

async fn login(
    app_state: Extension<AppState>,
    req: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    print!("logging in: {}", req.id);
    let mut conn = app_state.0.acquire_db_connection().await?;
    let rec = sqlx::query!(
        r#"
            SELECT password_hash
            FROM social_net.users
            WHERE user_id = $1
            "#,
        req.id,
    )
    .fetch_one(&mut conn)
    .await?;
    let input_password_hash = digest(req.password.clone());
    if rec.password_hash == input_password_hash {
        let token = format!("{}", Uuid::new_v4());
        print!("Success!");
        Ok(Json(LoginResponse { token }))
    } else {
        print!("Failed!");
        Err(ApiError::Unauthorized)
    }
}

async fn healthcheck(_app_state: Extension<AppState>) -> Result<(), ApiError> {
    println!("healthcheck call");

    Ok(())
}

pub async fn run(app_state: AppState) -> anyhow::Result<()> {
    let addr = app_state.cfg().http;

    let router = make_router(app_state);
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

fn make_router(app_state: AppState) -> Router {
    check_router().layer(Extension(app_state))
}

fn check_router() -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/login", post(login))
        .nest("/user/", user::user_router())
        .nest("/dialog/", dialog::dialog_router())
}
