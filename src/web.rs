use axum::extract::Path;
use axum::{
    body,
    extract::Json,
    Server,
    routing::{get, post},
    Router,
    Extension,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::{state::AppState};
use crate::error::ApiError;
use uuid::Uuid;

enum MyError {
    SomethingWentWrong,
    SomethingElseWentWrong,
}



#[derive (Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct GetUserResponse{
  id: i64,
  user_id: String,
  first_name: String,
  second_name: String,
  birthdate: Option<DateTime<Utc>>,
  biography: Option<String>,
  city: Option<String>
}

#[derive (Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct RegisterRequest {
  first_name: String,
  second_name: String,
  birthdate: Option<DateTime<Utc>>,
  biography: String,
  city: String,
  password: String,

}

#[derive(Serialize)]
pub struct RegisterResponse {
    user_id: String,
}

async fn get_user(
    Path(user_id): Path<String>,
    app_state: Extension<AppState>,
) -> Result<(), ApiError> {
    format!("get {user_id}");
    let mut conn = app_state.0.acquire_db_connection().await?;
    let _ = sqlx::query_as!(
            GetUserResponse,
            r#"
            SELECT id, user_id, first_name, second_name, birthdate, biography, city
            FROM social_net.users
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_one(&mut conn)
        .await?;
    Ok(())
    
}

async fn login(
    Path(user_id): Path<String>,
    app_state: Extension<AppState>,
) -> Result<(), ApiError> {
    print!("get {user_id}");
    let mut conn = app_state.0.acquire_db_connection().await?;
    let _ = sqlx::query_as!(
            GetUserResponse,
            r#"
            SELECT id, user_id, first_name, second_name, birthdate, biography, city
            FROM social_net.users
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_one(&mut conn)
        .await?;
    Ok(())
    
}

#[axum_macros::debug_handler]
async fn register(
    app_state: Extension<AppState>,
    req: Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    println!("register call");

    let user_id = Uuid::new_v4();
    let mut conn = app_state.0.acquire_db_connection().await.unwrap();
    let _ = sqlx::query!(
            r#"
            INSERT INTO social_net.users (user_id, first_name, second_name, birthdate, biography, city, password_hash)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            format!("{user_id}"), //TODO: прикрутить встроенную поддержку
            req.first_name,
            req.second_name,
            req.birthdate,
            req.biography,
            req.city,
            req.password
        )
        .execute(&mut conn)
        .await.unwrap();
    println!("{user_id} registered!");
    // Ok(Json(RegisterResponse{user_id: format!("{user_id}")}))
    Json(RegisterResponse{user_id: format!("{user_id}")})
    
}

pub async fn run(app_state: AppState) -> anyhow::Result<()> {
    let addr = app_state.cfg().http;

    let router = make_router(app_state);
    Server::bind(&addr).serve(router.into_make_service()).await?;

    Ok(())
}


fn make_router(app_state: AppState) -> Router {
    check_router(app_state.clone())
        .layer(Extension(app_state))
}

fn check_router(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/login",
            post(|| async { "login" })
        )
        .route(
            "/user/register",
            post(register)
        )
        .route(
            "/user/get/:id",
            get(get_user)
        )
}