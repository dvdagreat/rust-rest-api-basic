use std::error::Error;

use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::{
    controllers::users::UsersController, repositories::users::UsersRepository,
    services::users::UsersService,
};

pub mod controllers;
pub mod db;
pub mod repositories;
pub mod services;

#[derive(Serialize)]
struct EndpointResponse {
    message: String,
}

async fn get_user_handler() -> impl IntoResponse {
    let response = EndpointResponse {
        message: "hello world".to_string(),
    };

    Json(response)
}

fn create_app() -> Router {
    Router::new().route("/users", get(get_user_handler))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let database_connection = db::connection::get_connection().await?;
    let users_repository = UsersRepository::new(database_connection.clone());
    let users_service = UsersService::new(users_repository);
    let users_controller = UsersController::new(users_service);

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234")
        .await
        .expect("Cannot bind the tcp listener");

    axum::serve(listener, app)
        .await
        .expect("Cannot start axum app");

    Ok(())
}
