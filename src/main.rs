use axum::{
    extract::{Host, Request, Query},
    http::StatusCode,
    http::uri::Scheme,
    response::Json,
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};
use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
struct AppState {
    oauth_client: BasicClient,
}

#[derive(Deserialize)]
struct AuthCallbackQuery {
    code: String,
}

#[derive(Serialize)]
struct AuthResponse {
    access_token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID")
            .expect("GOOGLE_CLIENT_ID must be set")
    );
    let client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("GOOGLE_CLIENT_SECRET must be set")
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:3000/auth/callback".to_string())
                .expect("Invalid redirect URL"),
        );

    let state = AppState {
        oauth_client: client,
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false); // Для разработки, в продакшене true

    let app = Router::new()
        .route("/auth/login", get(auth_login))
        .route("/auth/callback", get(auth_callback))
        .route("/protected", get(protected_route))
        .route("/base-url", get(get_base_url))
         .route("/get_base_url_with_proxy", get(get_base_url_with_proxy))
        .layer(CorsLayer::permissive())
        .layer(session_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn auth_login(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> (StatusCode, Json<serde_json::Value>) {
    let (auth_url, _csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "auth_url": auth_url.to_string()
        })),
    )
}

async fn auth_callback(
    session: Session,
    Query(params): Query<AuthCallbackQuery>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> (StatusCode, Json<AuthResponse>) {
    let code = oauth2::AuthorizationCode::new(params.code);
    let token_result = state
        .oauth_client
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret().clone();
            session.insert("access_token", access_token).await.unwrap();

            (
                StatusCode::OK,
                Json(AuthResponse {
                    access_token: "authenticated".to_string(), // Indicate successful authentication
                }),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                access_token: "".to_string(),
            }),
        ),
    }
}

async fn protected_route(
    session: Session,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Ok(Some(_access_token)) = session.get::<String>("access_token").await {
        (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Access granted"})),
        )
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
    }
}

async fn get_base_url(
    Host(host): Host,
    request: Request,
) -> Json<serde_json::Value> {
    // Определяем схему (http/https)
    let scheme = match request.uri().scheme() {
        Some(scheme) if scheme == &Scheme::HTTPS => "https",
        _ => "http", // По умолчанию http, для https проверьте заголовки прокси
    };
    
    let base_url = format!("{}://{}", scheme, host);
    
    Json(serde_json::json!({
        "base_url": base_url
    }))
}

async fn get_base_url_with_proxy(
    Host(host): Host,
    request: Request,
) -> Json<serde_json::Value> {
    let scheme = if let Some(proto) = request.headers().get("x-forwarded-proto") {
        proto.to_str().unwrap_or("http")
    } else {
        match request.uri().scheme() {
            Some(scheme) if scheme == &Scheme::HTTPS => "https",
            _ => "http",
        }
    };
    
    let base_url = format!("{}://{}", scheme, host);
    
    Json(serde_json::json!({
        "base_url": base_url
    }))
}