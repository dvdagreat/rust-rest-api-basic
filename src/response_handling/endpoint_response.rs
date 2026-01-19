use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub data: T,
}

impl<T> SuccessResponse<T>
where
    T: Serialize,
{
    pub fn new(message: impl Into<String>, data: T) -> Self {
        Self {
            message: message.into(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
