use sea_orm::sea_query::error::Error;

use crate::repositories::users::UsersRepository;

pub struct UsersService {
    users_repository: UsersRepository,
}

impl UsersService {
    pub fn new(users_repository: UsersRepository) -> Self {
        Self { users_repository }
    }

    pub fn find_user_by_id(id: i64) -> Result<(), Error> {
        println!("id: {}", id);
        Ok(())
    }

    pub fn create_user(id: i64) -> Result<(), Error> {
        println!("id: {}", id);
        Ok(())
    }

    pub fn delete_user(id: i64) -> Result<(), Error> {
        println!("id: {}", id);
        Ok(())
    }

    pub fn update_user(id: i64, model: i64) -> Result<(), Error> {
        println!("id: {}, model: {}", id, model);
        Ok(())
    }
}
