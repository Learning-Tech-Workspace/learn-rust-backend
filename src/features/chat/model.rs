use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Chat {
    pub user_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub group_id: Uuid,
}

#[derive(Deserialize)]
pub enum MessageType {
    File,
    Text,
    Image,
}