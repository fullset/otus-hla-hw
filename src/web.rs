use axum::{Server,
    routing::post,
    Router,
    Extension,
};

use crate::{state::AppState};

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
            "/v1/check",
            post(|| async { "Hello, World!" })
        )
}