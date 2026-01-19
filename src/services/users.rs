use std::sync::Arc;

use crate::{
    db::entities::users::Model as UserModel, dtos::update_user_by_id::UpdateUserRequest,
    repositories::users::UsersRepository, response_handling::errors::ApplicationErrors,
};

pub struct UsersService {
    users_repository: Arc<UsersRepository>,
}

impl UsersService {
    pub fn new(users_repository: Arc<UsersRepository>) -> Self {
        Self { users_repository }
    }

    pub async fn search_users(
        &self,
        limit: i32,
        page: i32,
    ) -> Result<Vec<UserModel>, ApplicationErrors> {
        Ok(self.users_repository.search_users(limit, page).await?)
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<UserModel, ApplicationErrors> {
        Ok(self.users_repository.find_user_by_id(id).await?)
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
    ) -> Result<UserModel, ApplicationErrors> {
        Ok(self.users_repository.create_user(username, email).await?)
    }

    pub async fn delete_user(&self, id: i32) -> Result<u64, ApplicationErrors> {
        let result = self.users_repository.delete_user(id).await?;

        if result == 0 {
            return Err(ApplicationErrors::NotFound);
        }

        Ok(result)
    }

    pub async fn update_user(
        &self,
        id: i32,
        payload: UpdateUserRequest,
    ) -> Result<UserModel, ApplicationErrors> {
        Ok(self.users_repository.update_user(id, payload).await?)
    }
}
