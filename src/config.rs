use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub path: String,
    pub cache_size_mb: usize,
    pub flush_interval_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

const DEFAULT_WORKERS: usize = 4;
const DEFAULT_CACHE_SIZE: usize = 64;
const DEFAULT_FLUSH_INTERVAL: u64 = 1000;
const DEFAULT_LOG_LEVEL: &str = "info";

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
        let workers = env::var("SERVER_WORKERS")
            .map(|v| v.parse().unwrap_or(DEFAULT_WORKERS))
            .unwrap_or(DEFAULT_WORKERS);

        Ok(Self {
            server: ServerConfig {
                host: IpAddr::from_str(&host)?,
                port: port.parse()?,
                workers,
            },
            storage: StorageConfig {
                path: env::var("STORAGE_PATH").unwrap_or_else(|_| "url_db".to_string()),
                cache_size_mb: env::var("STORAGE_CACHE_SIZE_MB")
                    .map(|v| v.parse().unwrap_or(DEFAULT_CACHE_SIZE))
                    .unwrap_or(DEFAULT_CACHE_SIZE),
                flush_interval_ms: env::var("STORAGE_FLUSH_INTERVAL_MS")
                    .map(|v| v.parse().unwrap_or(DEFAULT_FLUSH_INTERVAL))
                    .unwrap_or(DEFAULT_FLUSH_INTERVAL),
            },
            logging: LoggingConfig {
                level: env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string()),
            },
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server.host, self.server.port)
    }

    pub fn server_url(&self) -> String {
        format!("http://{}", self.socket_addr())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Failed to load default configuration")
    }
} 