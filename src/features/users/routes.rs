use axum::{routing::get, Router};

use super::handler::{
    create_user, delete_user, get_all_users, get_user_by_id, update_avatar, update_user,
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route(
            "/:id",
            get(get_user_by_id)
                .post(update_avatar)
                .delete(delete_user)
                .patch(update_user),
        )
}
