use crate::services::users::UsersService;

pub struct UsersController {
    users_service: UsersService,
}

impl UsersController {
    pub fn new(users_service: UsersService) -> Self {
        Self { users_service }
    }
}
