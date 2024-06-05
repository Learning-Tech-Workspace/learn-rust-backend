use axum::http::header::AUTHORIZATION;
use handler::handler_disconnect;
use socketioxide::extract::SocketRef;
use tracing::info;

use crate::{
    error::{Error, Result},
    socket::handler::{handle_join, handle_message},
};

pub mod handler;
pub mod model;

pub async fn on_connect(socket: SocketRef) {
    info!("socket connected {}", socket.id);

    socket.on("message", handle_message);
    socket.on("join", handle_join);

    socket.on_disconnect(handler_disconnect)
}

// middleware
pub async fn check_login(socket: SocketRef) -> Result<()> {
    let token = socket
        .req_parts()
        .headers
        .get(AUTHORIZATION)
        .ok_or_else(|| Error::TokenNotFound)?
        .to_str()
        .or_else(|e| Err(Error::Unknown(e.to_string())))?;

    Ok(())
}
