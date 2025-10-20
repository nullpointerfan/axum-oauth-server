use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub redirect_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(".env").required(false))
            .add_source(config::Environment::default())
            .build()?;

        settings.try_deserialize()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            google_client_id: std::env::var("GOOGLE_CLIENT_ID")
                .unwrap_or_else(|_| "your_google_client_id_here".to_string()),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")
                .unwrap_or_else(|_| "your_google_client_secret_here".to_string()),
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            redirect_uri: std::env::var("REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
        }
    }
}