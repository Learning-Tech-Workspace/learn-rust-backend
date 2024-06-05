use axum::extract::Multipart;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use regex::Regex;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::error::{Error, Result};

use super::model::{CreateUser, UpdateUser, UserDTO};

use entity::user;

pub async fn create_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse> {
    let user_model = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(payload.password),
        is_online: Set(payload.is_online),
        ..Default::default()
    };

    user_model
        .insert(&db_connection)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok((
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "User created successfully"
            }
        )),
    ))
}

pub async fn get_user_by_id(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?;

    let result = UserDTO {
        id: user.id,
        name: user.name,
        email: user.email,
        avatar: user.avatar,
        is_online: user.is_online,
    };

    Ok((StatusCode::OK, Json(result)))
}

pub async fn update_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse> {
    let mut user: user::ActiveModel = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?
        .into();

    user.name = Set(payload.name.unwrap());
    user.email = Set(payload.email.unwrap());
    user.avatar = Set(payload.avatar);

    user.update(&db_connection)
        .await
        .map_err(|e| Error::UpdateFailed(e))?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "message": "User updated successfully"
            }
        )),
    ))
}

pub async fn delete_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?;

    user::Entity::delete_by_id(user.id)
        .exec(&db_connection)
        .await
        .map_err(|e| Error::DeleteFailed(e))?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(json!(
            {
                "message": "User deleted successfully"
            }
        )),
    ))
}

pub async fn get_all_users(
    Extension(db_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse> {
    let users: Vec<UserDTO> = user::Entity::find()
        .all(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .into_iter()
        .map(|user| UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
            is_online: user.is_online,
        })
        .collect();

    Ok((StatusCode::OK, Json(users)))
}

pub async fn update_avatar(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == "avatar" {
            let file_name = field.file_name().unwrap();
            let content_type = field.content_type().unwrap();

            let regex = Regex::new(mime::IMAGE_STAR.as_ref()).unwrap();

            if regex.is_match(&content_type) {
                let mut user: user::ActiveModel = user::Entity::find()
                    .filter(Condition::all().add(user::Column::Id.eq(id)))
                    .one(&db_connection)
                    .await
                    .map_err(|e| Error::QueryFailed(e))?
                    .ok_or_else(|| Error::RecordNotFound)?
                    .into();

                user.avatar = Set(Some(file_name.to_string()));

                let mut file = File::create(format!("./public/uploads/{file_name}"))
                    .await
                    .map_err(|_| Error::CreateFileFailed)?;
                let data = field.bytes().await.unwrap();
                file.write(&data).await.unwrap();

                user.update(&db_connection)
                    .await
                    .map_err(|e| Error::UpdateFailed(e))?;
            } else {
                return Err(Error::FileTypeInvalid);
            }
        }
    }

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Avatar updated successfully" })),
    ))
}
