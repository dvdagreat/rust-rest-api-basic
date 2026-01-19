use std::error::Error;
use std::sync::Arc;

use axum::Router;

use sea_orm::DatabaseConnection;

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
pub mod routers;
pub mod services;
pub mod state;

fn create_app(database_connection: DatabaseConnection) -> Router {
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
    let database_connection = db::connection::get_connection()
        .await
        .expect("Sorry! Cannot Connect to the database! Comeback later!");

    let app = create_app(database_connection.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234")
        .await
        .expect("Cannot bind the tcp listener");

    axum::serve(listener, app)
        .await
        .expect("Cannot start axum app");

    Ok(())
}
