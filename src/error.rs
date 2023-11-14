use thiserror::Error;
use axum::response::{Response, IntoResponse};
use axum::http::StatusCode;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error")]
    Db(#[from] sqlx::Error),
    #[error("Unknown error")]
    Unknown,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = match self {
            ApiError::Db(_)=> "Db error",
            ApiError::Unknown => "Unknown error",
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}