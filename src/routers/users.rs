use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::users::{
        create_user_handler, delete_user_by_id, find_user_by_id, search_users_handler,
        update_user_by_id,
    },
    state::AppState,
};

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(search_users_handler))
        .route("/", post(create_user_handler))
        .route("/{id}", get(find_user_by_id))
        .route("/{id}", delete(delete_user_by_id))
        .route("/{id}", put(update_user_by_id))
}
