use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::features::auth::middleware::check_login;

use super::handler::{create_group, get_group_by_id};

pub fn get_routes() -> Router {
    let protected_routes = Router::new()
        .route("/", post(create_group))
        .layer(middleware::from_fn(check_login));
    let public_routes = Router::new().route("/:id", get(get_group_by_id));

    Router::new().merge(public_routes).merge(protected_routes)
}
