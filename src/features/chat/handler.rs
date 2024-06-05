use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::error::Result;

use super::model::Chat;
use super::service::insert_chat;

pub async fn chat(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<Chat>,
) -> Result<impl IntoResponse> {
    insert_chat(db_connection, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "Chat created successfully"
            }
        )),
    ))
}
