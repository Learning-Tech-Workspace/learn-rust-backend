use entity::sea_orm_active_enums::MessageEnum;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use entity::{conversation, message};

use crate::error::{Error, Result};

use super::model::{Chat, MessageType};

pub async fn insert_chat(db_connection: DatabaseConnection, payload: Chat) -> Result<()> {
    let message_type = match payload.message_type {
        MessageType::File => MessageEnum::File,
        MessageType::Text => MessageEnum::Text,
        MessageType::Image => MessageEnum::Image,
    };

    let message_model = message::ActiveModel {
        user_id: Set(payload.user_id),
        content: Set(payload.content),
        r#type: Set(Some(message_type)),
        ..Default::default()
    };

    let message = message_model
        .insert(&db_connection)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    let conversation_model = conversation::ActiveModel {
        group_id: Set(payload.group_id),
        msg_id: Set(message.id),
    };

    conversation_model
        .insert(&db_connection)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok(())
}
