use std::sync::Arc;

use crate::services::users::UsersService;

#[derive(Clone)]
pub struct AppState {
    pub users: UserDependenciesState,
}

#[derive(Clone)]
pub struct UserDependenciesState {
    pub users_service: Arc<UsersService>,
}
