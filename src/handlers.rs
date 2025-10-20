use axum::{
    extract::Query,
    response::Json,
    Extension,
};
use oauth2::{AuthorizationCode, TokenResponse};
use tower_sessions::Session;

use crate::error::AppError;
use crate::models::{AppState, AuthCallbackQuery, AuthResponse, AuthUrlResponse};

pub async fn auth_login(
    Extension(state): Extension<AppState>,
) -> Result<Json<AuthUrlResponse>, AppError> {
    let (auth_url, _csrf_token) = state
        .oauth_client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("openid".to_string()))
        .add_scope(oauth2::Scope::new("email".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .url();

    Ok(Json(AuthUrlResponse {
        auth_url: auth_url.to_string(),
    }))
}

pub async fn auth_callback(
    session: Session,
    Query(params): Query<AuthCallbackQuery>,
    Extension(state): Extension<AppState>,
) -> Result<Json<AuthResponse>, AppError> {
    let code = AuthorizationCode::new(params.code);
    let token_result = state
        .oauth_client
        .exchange_code(code)
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret().clone();
            session.insert("access_token", access_token)
                .await
                .map_err(|_| AppError::Session("Failed to store session".to_string()))?;

            Ok(Json(AuthResponse {
                access_token: "authenticated".to_string(),
            }))
        }
        Err(e) => Err(AppError::OAuth2(format!("Token exchange failed: {:?}", e))),
    }
}

pub async fn protected_route(
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    match session.get::<String>("access_token").await {
        Ok(Some(_access_token)) => Ok(Json(serde_json::json!({"message": "Access granted"}))),
        _ => Err(AppError::InternalServerError),
    }
}