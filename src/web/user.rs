use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::Path;
use axum::{
    extract::{Json, Query},
    routing::{get, post},
    Extension, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sha256::digest;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct GetUserResponse {
    id: i64,
    user_id: String,
    first_name: String,
    second_name: String,
    birthdate: Option<NaiveDate>,
    biography: Option<String>,
    city: Option<String>,
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct RegisterRequest {
    first_name: String,
    second_name: String,
    birthdate: Option<NaiveDate>,
    biography: String,
    city: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    user_id: String, //TODO: Uuid
}

#[derive(Deserialize, Debug)]
pub struct SearchRequest {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "id")]
    user_id: String,
    first_name: String,
    second_name: String,
    birthdate: Option<NaiveDate>,
    biography: Option<String>,
    city: Option<String>,
}

async fn register(
    app_state: Extension<AppState>,
    req: Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, ApiError> {
    println!("register call");

    let user_id = Uuid::new_v4();
    let password_hash = digest(req.password.clone());
    let mut conn = app_state.0.acquire_db_connection().await?;
    let _ = sqlx::query!(
            r#"
            INSERT INTO social_net.users (user_id, first_name, second_name, birthdate, biography, city, password_hash)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            format!("{user_id}"), //TODO: прикрутить встроенную поддержку Uuid
            req.first_name,
            req.second_name,
            req.birthdate,
            req.biography,
            req.city,
            password_hash
        )
        .execute(&mut conn)
        .await?;
    println!("{user_id} registered!");
    Ok(Json(RegisterResponse {
        user_id: format!("{user_id}"),
    }))
}

async fn get_user(
    Path(user_id): Path<String>,
    app_state: Extension<AppState>,
) -> Result<Json<GetUserResponse>, ApiError> {
    println!("get {user_id}");
    let mut conn = app_state.0.acquire_db_connection().await?;
    let res = sqlx::query_as!(
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
    Ok(Json(res))
}

async fn search_user(
    Query(search_req): Query<SearchRequest>,
    app_state: Extension<AppState>,
) -> Result<Json<Vec<SearchResponse>>, ApiError> {
    println!("search {:?}", search_req);
    let mut conn = app_state.0.acquire_db_connection().await?;
    let res: Vec<SearchResponse> = sqlx::query_as!(
        SearchResponse,
        r#"
            SELECT user_id, first_name, second_name, birthdate, biography, city
            FROM social_net.users
            WHERE upper(first_name) LIKE upper($1) AND upper(second_name) LIKE upper($2)
            "#,
        format!("{}%", search_req.first_name),
        format!("{}%", search_req.last_name),
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(res))
}

pub fn user_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/get/:id", get(get_user))
        .route("/search", get(search_user))
}
