use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use entity::user;

use crate::{
    error::{Error, Result},
    utils::jwt,
};

use super::model::{LoginRequest, LoginResponse};

pub async fn login(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let LoginRequest { email, password } = payload;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .filter(user::Column::Password.eq(password))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or(Error::RecordNotFound)?;

    let token = jwt::encode_jwt(user.id)?;

    let resp = LoginResponse {
        msg: String::from("Login Successfully!"),
        token: token,
    };

    Ok((StatusCode::ACCEPTED, Json(resp)))
}