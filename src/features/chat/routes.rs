use axum::{routing::post, Router};

use super::handler::chat;

pub fn get_routes() -> Router {
    Router::new().route("/", post(chat))
}
