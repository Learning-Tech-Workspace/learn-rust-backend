use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::features::users::model::UserDTO;

use super::model::{CreateGroup, GroupDTO};

use entity::{group, user, user_group};

pub async fn create_group(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<CreateGroup>,
) -> Result<impl IntoResponse> {
    let group_model = group::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };
    let new_group = group_model
        .insert(&db_connection)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    let records: Vec<user_group::ActiveModel> = payload
        .user_ids
        .into_iter()
        .map(|user_id| user_group::ActiveModel {
            group_id: Set(new_group.id),
            user_id: Set(user_id),
        })
        .collect();

    user_group::Entity::insert_many(records)
        .exec(&db_connection)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok((
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "Group created successfully"
            }
        )),
    ))
}

pub async fn get_group_by_id(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let group = group::Entity::find()
        .filter(Condition::all().add(group::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?;

    let user_ids: Vec<Uuid> = user_group::Entity::find()
        .filter(Condition::all().add(user_group::Column::GroupId.eq(group.id)))
        .all(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .into_iter()
        .map(|user_group_model| user_group_model.user_id)
        .collect();

    let mut users: Vec<UserDTO> = vec![];
    for user_id in user_ids.into_iter() {
        let user = user::Entity::find()
            .filter(Condition::all().add(user::Column::Id.eq(user_id)))
            .one(&db_connection)
            .await
            .map_err(|e| Error::QueryFailed(e))?
            .ok_or_else(|| Error::RecordNotFound)?;
        users.push(UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
            is_online: user.is_online,
        });
    }

    let result = GroupDTO {
        id: group.id,
        users,
    };

    Ok((StatusCode::OK, Json(result)))
}
