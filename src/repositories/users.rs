use crate::{
    db::entities::users::{
        ActiveModel as UsersActiveModel, Entity as UsersEntity, Model as UsersModel,
    },
    dtos::update_user_by_id::UpdateUserRequest,
    response_handling::errors::ApplicationErrors,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

pub struct UsersRepository {
    db_connection: DatabaseConnection,
}

impl UsersRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db_connection: db }
    }

    pub async fn search_users(
        &self,
        limit: i32,
        page: i32,
    ) -> Result<Vec<UsersModel>, ApplicationErrors> {
        let result = UsersEntity::find().all(&self.db_connection).await;

        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(ApplicationErrors::InternalError),
        }
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<UsersModel, ApplicationErrors> {
        let user = UsersEntity::find_by_id(id)
            .one(&self.db_connection)
            .await
            .map_err(|_| ApplicationErrors::InternalError)?;

        user.ok_or(ApplicationErrors::NotFound)
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
    ) -> Result<UsersModel, ApplicationErrors> {
        let user_model = UsersActiveModel {
            username: Set(username.to_owned()),
            email: Set(email.to_owned()),
            ..Default::default()
        }
        .insert(&self.db_connection)
        .await;

        match user_model {
            Ok(user_model) => Ok(user_model),
            Err(err) => {
                println!("{}", err);
                Err(ApplicationErrors::InternalError)
            }
        }
    }

    pub async fn delete_user(&self, id: i32) -> Result<u64, ApplicationErrors> {
        let result = UsersEntity::delete_by_id(id)
            .exec(&self.db_connection)
            .await
            .map_err(|_| ApplicationErrors::InternalError)?;
        Ok(result.rows_affected)
    }

    pub async fn update_user(
        &self,
        id: i32,
        model: UpdateUserRequest,
    ) -> Result<UsersModel, ApplicationErrors> {
        let user = UsersEntity::find_by_id(id)
            .one(&self.db_connection)
            .await
            .map_err(|_| ApplicationErrors::InternalError)?
            .ok_or(ApplicationErrors::NotFound)?;

        let mut user: UsersActiveModel = user.into();

        user.username = Set(model.username);
        user.email = Set(model.email);

        let updated = user
            .update(&self.db_connection)
            .await
            .map_err(|_| ApplicationErrors::InternalError)?;
        Ok(updated)
    }
}
