use serde::{Deserialize, Serialize};
use uuid::Uuid;

// for FE to render the message
#[derive(Debug, Serialize)]
pub struct MessageOut {
    pub content: String,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinRoom {
    pub room: Uuid, // group_id
}
