use sea_orm::DatabaseConnection;
use socketioxide::{
    extract::{Data, SocketRef},
    socket::DisconnectReason,
};
use tracing::info;

use crate::{
    error::Error,
    features::chat::{model::Chat, service::insert_chat},
    socket::model::MessageOut,
};

use super::model::JoinRoom;

pub async fn handle_message(socket: SocketRef, Data(data): Data<Chat>) {
    let db_connection = socket
        .req_parts()
        .extensions
        .get::<DatabaseConnection>()
        .ok_or_else(|| Error::DatabaseConnectionFailed)
        .unwrap();

    let _ = insert_chat(db_connection.to_owned(), data.clone()).await;

    let resp = MessageOut {
        content: data.content,
        user_id: data.user_id,
        group_id: data.group_id,
        created_at: chrono::Utc::now(),
    };

    socket
        .within(data.group_id.to_string())
        .emit("message-back", resp)
        .ok();
}

pub fn handle_join(socket: SocketRef, Data(data): Data<JoinRoom>) {
    info!("Received join: {:?}", data);

    let _ = socket.leave_all(); // before joining a new room, leave all rooms
    let _ = socket.join(data.room.to_string());

    socket
        .within(data.room.to_string())
        .emit("join-room-back", data)
        .ok();
}

pub async fn handler_disconnect(socket: SocketRef, reason: DisconnectReason) {
    println!("Socket {} was disconnected because {} ", socket.id, reason);
}
