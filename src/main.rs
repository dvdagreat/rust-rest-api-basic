use std::error::Error;

use crate::{
    controllers::users::UsersController, repositories::users::UsersRepository,
    services::users::UsersService,
};

pub mod controllers;
pub mod db;
pub mod repositories;
pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let database_connection = db::connection::get_connection().await?;
    let users_repository = UsersRepository::new(database_connection.clone());
    let users_service = UsersService::new(users_repository);
    let users_controller = UsersController::new(users_service);

    Ok(())
}
