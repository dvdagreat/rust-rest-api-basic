use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub username: String,
}
