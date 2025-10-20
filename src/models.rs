use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    pub code: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct AuthUrlResponse {
    pub auth_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub oauth_client: oauth2::basic::BasicClient,
}