use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;

use super::model::{CreateUser, UpdateUser, UserDTO};

use entity::user;

pub async fn create_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user_model = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(payload.password),
        is_online: Set(payload.is_online),
        ..Default::default()
    };

    user_model.insert(&db_connection).await.unwrap();

    (
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "User created successfully"
            }
        )),
    )
}

pub async fn get_user_by_id(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .unwrap()
        .unwrap();

    let result = UserDTO {
        id: user.id,
        name: user.name,
        email: user.email,
        avatar: user.avatar,
        is_online: user.is_online,
    };

    (StatusCode::CREATED, Json(result))
}

pub async fn update_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let mut user: user::ActiveModel = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .unwrap()
        .unwrap() 
        .into();

    user.name = Set(payload.name.unwrap());
    user.email = Set(payload.email.unwrap());
    user.avatar = Set(payload.avatar);

    user.update(&db_connection).await.unwrap();

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "message": "User updated successfully"
            }
        )),
    )
}

pub async fn delete_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let mut user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .unwrap()
        .unwrap();

    user::Entity::delete_by_id(user.id)
        .exec(&db_connection)
        .await
        .unwrap();

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "message": "User deleted successfully"
            }
        )),
    )
}

pub async fn get_all_users(
    Extension(db_connection): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let users: Vec<UserDTO> = user::Entity::find()
        .all(&db_connection)
        .await
        .unwrap()
        .into_iter()
        .map(|user| UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
            is_online: user.is_online,
        })
        .collect();

    (StatusCode::ACCEPTED, Json(users))
}
