use axum::{middleware, routing::post, Router};

use crate::features::auth::middleware::check_login;

use super::handler::chat;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", post(chat))
        .layer(middleware::from_fn(check_login))
}
