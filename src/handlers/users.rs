use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    AppState,
    db::entities::users::Model as UsersModel,
    dtos::{create_user::CreateUserRequest, update_user_by_id::UpdateUserRequest},
    response_handling::{endpoint_response::SuccessResponse, errors::ApplicationErrors},
};

pub async fn search_users_handler(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Vec<UsersModel>>>, ApplicationErrors> {
    let users = state.users.users_service.search_users(100, 100).await?;

    Ok(Json(SuccessResponse::new("Users fetched", users)))
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<SuccessResponse<UsersModel>>, ApplicationErrors> {
    let user = state
        .users
        .users_service
        .create_user(payload.username, payload.email)
        .await?;

    Ok(Json(SuccessResponse {
        message: "User created".to_owned(),
        data: user,
    }))
}

pub async fn find_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<SuccessResponse<UsersModel>>, ApplicationErrors> {
    let user = state.users.users_service.find_user_by_id(id).await?;
    Ok(Json(SuccessResponse {
        message: "User found".to_owned(),
        data: user,
    }))
}

pub async fn delete_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<SuccessResponse<u64>>, ApplicationErrors> {
    let rows_affected = state.users.users_service.delete_user(id).await?;
    Ok(Json(SuccessResponse {
        message: "User deleted".to_owned(),
        data: rows_affected,
    }))
}

pub async fn update_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<SuccessResponse<UsersModel>>, ApplicationErrors> {
    let model = state.users.users_service.update_user(id, payload).await?;
    Ok(Json(SuccessResponse {
        message: "User updated".to_owned(),
        data: model,
    }))
}
