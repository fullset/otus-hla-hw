use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::Path;
use axum::{
    extract::Json,
    routing::{get, post},
    Extension, Router,
};
use axum_macros::debug_handler;

use serde::{Deserialize, Serialize};

use sqlx::FromRow;

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct SendRequest {
    text: String,
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct ListResponse {
    #[serde(rename = "from")]
    user_from: Option<String>,
    #[serde(rename = "to")]
    user_to: Option<String>,
    message: Option<String>,
}

#[debug_handler]
async fn send(
    Path(user_id): Path<String>,
    app_state: Extension<AppState>,
    req: Json<SendRequest>,
) -> Result<(), ApiError> {
    println!("send {user_id}");
    // TODO: разбивать сообщение на подсообщения, чтобы избежать переполнения типа в БД.
    let mut conn = app_state.0.acquire_db_connection().await?;
    // NOTE: Это явно не описано в спеке и в задании по шардированию.
    // Видимо, user_id пользователя, отправляющего сообщение, должен быть сохранен, когда тот логинится.
    // Однако реализация выходит за рамки задания, поэтому будет использавать захардкоженное значение.
    let _res = sqlx::query!(
        r#"
            INSERT INTO social_net.messages (user_from, user_to, message, ts)
            VALUES ($1, $2, $3, NOW())
        "#,
        "123", // TODO: здесь должен быть честный id пользователя, сохраненный при вызове метода /login
        user_id,
        req.text,
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

async fn list(
    Path(user_id): Path<String>,
    app_state: Extension<AppState>,
) -> Result<Json<Vec<ListResponse>>, ApiError> {
    println!("list {user_id}");
    let mut conn = app_state.0.acquire_db_connection().await?;

    // NOTE: Это явно не описано в спеке и в задании по шардированию.
    // Видимо, user_id пользователя, отправляющего сообщение, должен быть сохранен, когда тот логинится.
    // Однако реализация выходит за рамки задания, поэтому будет использавать захардкоженное значение.
    let res: Vec<ListResponse> = sqlx::query_as!(
        ListResponse,
        r#"
            SELECT user_from, user_to, message
            FROM social_net.messages WHERE user_from = $1 AND user_to = $2
            UNION 
            SELECT user_from, user_to, message
            FROM social_net.messages WHERE user_from = $2 AND user_to = $1
            "#,
        "123", // TODO: здесь должен быть честный id пользователя, сохраненный при вызове метода /login
        user_id,
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(res))
}

pub fn dialog_router() -> Router {
    Router::new()
        .route("/:user_id/send", post(send))
        .route("/get/:user_id/list", get(list))
}
