use sea_orm::{DatabaseConnection, sea_query::error::Error};

pub struct UsersRepository {
    db: DatabaseConnection,
}

impl UsersRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
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
