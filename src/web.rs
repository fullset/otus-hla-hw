use chrono::NaiveDate;

use axum::extract::Path;
use axum::{Server,
    routing::{get, post},
    Router,
    Extension,
};

use crate::{state::AppState};

pub struct GetUserResponse{
  id: i64,
  user_id: String,
  first_name: String,
  second_name: String,
  birthdate: Option<NaiveDate>,
  biography: Option<String>,
  city: Option<String>
}

async fn get_user(
    Extension(app_state): Extension<AppState>,
    Path(user_id): Path<String>,
) -> anyhow::Result<GetUserResponse> {
    format!("get {user_id}");
    let conn = app_state.acquire_db_connection().await?;
    let rec = sqlx::query_as!(
            GetUserResponse,
            r#"
            SELECT id, user_id, first_name, second_name, birthdate, biography, city
            FROM social_net.users
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_one(conn)
        .await?;
    Ok(rec)
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
            post(|| async { "register" })
        )
        .route(
            "/user/get/:id",
            get(get_user)
        )
}