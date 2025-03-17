use std::sync::Arc;

use crate::{
    db::UserExt,
    dtos::{
        EmailListResponseDto, FilterEmailDto, FilterUserDto, NameUpdateDto, Response,
        SearchQueryByEmailDTO, UserData, UserPasswordUpdateDto, UserResponseDto,
    },
    error::{ErrorMessage, HttpError},
    middleware::JWTAuthMiddleware,
    utils::password,
    AppState,
};
use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use validator::Validate;

pub fn users_handler() -> Router {
    Router::new()
        .route("/me", get(get_me))
        .route("/name", put(update_user_name))
        .route("/password", put(update_user_password))
        .route("/search-emails", get(search_by_email))
}

pub async fn get_me(
    Extension(_app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user.user);

    let response_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: filtered_user,
        },
    };

    Ok(Json(response_data))
}

pub async fn update_user_name(
    Extension(app_state): Extension<Arc<AppState>>, 
    Extension(user): Extension<JWTAuthMiddleware>,
    Json(body): Json
)
