use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Environment variable errors
    #[error("Environment variable {0} not found")]
    EnvVarNotFound(String),

    // Database errors
    #[error("Database connection failed")]
    DatabaseConnectionFailed,
    #[error("Insert failed: {0}")]
    InsertFailed(#[source] sea_orm::error::DbErr),
    #[error("Query failed {0}")]
    QueryFailed(#[source] sea_orm::error::DbErr),
    #[error("Update failed: {0}")]
    UpdateFailed(#[source] sea_orm::error::DbErr),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Delete failed: {0}")]
    DeleteFailed(#[source] sea_orm::error::DbErr),

    // File errors
    #[error("Create file failed")]
    CreateFileFailed,

    #[error("File type invalid")]
    FileTypeInvalid,

    // JWT errors
    #[error("JWT decode failed: {0}")]
    DecodeJwtFailed(String),

    // Auth errors
    #[error("Please login first")]
    TokenNotFound,

    // Other errors
    #[error("{0}")]
    Unknown(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResp {
            status: String,
            message: String,
        }

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResp {
                status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}
