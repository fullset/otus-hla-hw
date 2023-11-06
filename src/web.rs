use axum::extract::Path;
use axum::{Server,
    routing::post,
    Router,
    Extension,
};

use crate::{state::AppState};

async fn get_user(
    Path(user_id): Path<String>,
) -> std::string::String {
    format!("get {user_id}")
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
            post(get_user)
        )
}