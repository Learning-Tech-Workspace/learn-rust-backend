use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct Chat {
    pub user_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub group_id: Uuid,
}

#[derive(Deserialize, Debug, Clone)]
pub enum MessageType {
    File,
    Text,
    Image,
}