use axum::{routing::post, Router};

use super::handler::login;

pub fn get_routes() -> Router {
    Router::new().route("/login", post(login))
}
