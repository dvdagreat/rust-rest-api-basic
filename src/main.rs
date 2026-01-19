use std::error::Error;
use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::users::{
        create_user_handler, delete_user_by_id, find_user_by_id, search_users_handler,
        update_user_by_id,
    },
    repositories::users::UsersRepository,
    services::users::UsersService,
    state::{AppState, UserDependenciesState},
};

pub mod db;
pub mod dtos;
pub mod handlers;
pub mod repositories;
pub mod response_handling;
pub mod services;
pub mod state;

async fn create_app() -> Router {
    let database_connection = db::connection::get_connection()
        .await
        .expect("Cannot Connect to the database! Sorry comeback later! haha");
    let users_repository = Arc::new(UsersRepository::new(database_connection.clone()));
    let users_service = Arc::new(UsersService::new(users_repository));

    let state = AppState {
        users: UserDependenciesState { users_service },
    };

    Router::new()
        .route("/users", get(search_users_handler))
        .route("/users", post(create_user_handler))
        .route("/users/{id}", get(find_user_by_id))
        .route("/users/{id}", delete(delete_user_by_id))
        .route("/users/{id}", put(update_user_by_id))
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let app = create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234")
        .await
        .expect("Cannot bind the tcp listener");

    axum::serve(listener, app)
        .await
        .expect("Cannot start axum app");

    Ok(())
}
