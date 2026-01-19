use std::error::Error;
use std::sync::Arc;

use axum::Router;

use crate::{
    repositories::users::UsersRepository,
    routers::users::create_user_router,
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
        .nest("/users", create_user_router())
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
