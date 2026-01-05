use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub server_host: String,
    pub server_port: u16,
    // Telemetry
    pub otlp_endpoint: Option<String>,
    pub service_name: String,
    pub json_logs: bool,
    pub metrics_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS must be a valid integer"),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid port number"),
            // Telemetry
            otlp_endpoint: env::var("OTLP_ENDPOINT").ok(),
            service_name: env::var("SERVICE_NAME").unwrap_or_else(|_| "xync-server".to_string()),
            json_logs: env::var("JSON_LOGS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            metrics_port: env::var("METRICS_PORT")
                .unwrap_or_else(|_| "9090".to_string())
                .parse()
                .expect("METRICS_PORT must be a valid port number"),
        }
    }
}
