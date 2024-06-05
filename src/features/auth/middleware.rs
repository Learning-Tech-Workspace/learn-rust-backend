use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, HeaderMap},
    middleware::Next,
    response::IntoResponse,
};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    error::{Error, Result},
    utils::jwt::decode_jwt,
};

use entity::user;

pub async fn check_login(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    let token = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| Error::TokenNotFound)?
        .to_str()
        .or_else(|e| Err(Error::Unknown(e.to_string())))?;

    let token = token.replace("Bearer ", "");

    let user_id = decode_jwt(token)?;

    let db_connection = req
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| Error::DatabaseConnectionFailed)?;

    let user = user::Entity::find_by_id(user_id)
        .one(db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?;

    req.extensions_mut().insert(user.id);

    let res = next.run(req).await;

    Ok(res)
}
