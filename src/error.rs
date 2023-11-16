use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error")]
    Db(#[from] sqlx::Error),
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        print!("{:?}", self);
        match self {
            ApiError::Db(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "Not found").into_response()
            }
            ApiError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Db error").into_response(),
            ApiError::Unauthorized => (StatusCode::FORBIDDEN, "Unauthorized").into_response(),
        }
    }
}
