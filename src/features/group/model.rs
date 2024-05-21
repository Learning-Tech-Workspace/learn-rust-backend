use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::users::model::UserDTO;

#[derive(Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub user_ids: Vec<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct GroupDTO {
    pub id: Uuid,
    pub users: Vec<UserDTO>,
}
