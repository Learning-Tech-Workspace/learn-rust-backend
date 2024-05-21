use axum::{
    routing::{get, post},
    Router,
};

use super::handler::{create_group, get_group_by_id};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", post(create_group))
        .route("/:id", get(get_group_by_id))
}
