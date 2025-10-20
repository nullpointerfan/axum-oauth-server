mod config;
mod error;
mod handlers;
mod models;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, models::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_oauth_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv().ok();

    // Load configuration
    let config = Config::from_env().unwrap_or_else(|_| {
        tracing::warn!("Failed to load config from environment, using defaults");
        Config::default()
    });

    // Create OAuth2 client
    let client_id = oauth2::ClientId::new(config.google_client_id);
    let client_secret = oauth2::ClientSecret::new(config.google_client_secret);
    let auth_url = oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?;
    let token_url = oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

    let oauth_client =
        oauth2::basic::BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(oauth2::RedirectUrl::new(config.redirect_uri)?);

    let state = AppState { oauth_client };

    // Create session store
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false); // For development, set to true in production

    // Build the application
    let app = Router::new()
        .route("/auth/login", get(handlers::auth_login))
        .route("/auth/callback", get(handlers::auth_callback))
        .route("/protected", get(handlers::protected_route))
        .layer(CorsLayer::permissive())
        .layer(session_layer)
        .layer(axum::Extension(state));

    // Start the server
    let addr = format!("{}:{}", config.server_host, config.server_port)
        .parse::<SocketAddr>()?;
    tracing::info!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
