mod api;
mod content;
mod db;
mod domain;
mod error;

use std::path::{Path, PathBuf};

pub use api::router;
pub use db::AppState;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_path: PathBuf,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let host = std::env::var("MATHFORGE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        if !matches!(host.as_str(), "127.0.0.1" | "localhost" | "::1") {
            return Err(
                "MATHFORGE_HOST darf nur eine Loopback-Adresse sein: 127.0.0.1, localhost oder ::1."
                    .to_string(),
            );
        }
        let port = match std::env::var("MATHFORGE_PORT") {
            Ok(raw) => raw
                .parse::<u16>()
                .map_err(|_| "MATHFORGE_PORT muss eine ganze Zahl von 0 bis 65535 sein.".to_string())?,
            Err(_) => 3001,
        };
        let database_path = match std::env::var("MATHFORGE_DB") {
            Ok(raw) => PathBuf::from(raw),
            Err(_) => PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join("mathforge.db"),
        };
        let cors_raw = std::env::var("MATHFORGE_CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://127.0.0.1:3000".to_string());
        let cors_origins = cors_raw
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        if cors_origins.is_empty() {
            return Err("MATHFORGE_CORS_ORIGINS enthält keinen Origin.".to_string());
        }
        Ok(Self {
            host,
            port,
            database_path,
            cors_origins,
        })
    }
}

pub fn app_from_path(path: &Path, origins: &[String]) -> Result<axum::Router, String> {
    let conn = db::open(path).map_err(|err| err.message)?;
    let state = AppState {
        db: std::sync::Arc::new(std::sync::Mutex::new(conn)),
    };
    router(state, origins)
}

pub fn serve(config: Config) -> Result<(), String> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|err| format!("Tokio konnte nicht starten: {err}"))?;
    runtime.block_on(run(config))
}

async fn run(config: Config) -> Result<(), String> {
    let app = app_from_path(&config.database_path, &config.cors_origins)?;
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|err| format!("Bind auf {addr} fehlgeschlagen: {err}"))?;
    println!("MathForge API hört auf http://{addr}");
    println!("Datenbank: {}", config.database_path.display());
    axum::serve(listener, app)
        .await
        .map_err(|err| format!("Server beendet: {err}"))
}
