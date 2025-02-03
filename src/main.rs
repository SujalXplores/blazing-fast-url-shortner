use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use std::sync::Arc;
use tracing::{info, debug, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::error::Error as StdError;
use std::fmt;

mod handlers;
mod config;
mod services;
mod repositories;

use config::Config;
use handlers::url_handler::{shorten_url, redirect, health_check};
use repositories::url_repository::SledUrlRepository;
use services::url_service::{UrlService, UrlServiceError};

#[derive(Debug)]
enum AppError {
    Config(Box<dyn StdError>),
    Storage(sled::Error),
    Service(UrlServiceError),
    Server(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(e) => write!(f, "Configuration error: {}", e),
            Self::Storage(e) => write!(f, "Storage error: {}", e),
            Self::Service(e) => write!(f, "Service error: {}", e),
            Self::Server(e) => write!(f, "Server error: {}", e),
        }
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Config(e) => Some(e.as_ref()),
            Self::Storage(e) => Some(e),
            Self::Service(e) => Some(e),
            Self::Server(e) => Some(e),
        }
    }
}

impl From<Box<dyn StdError>> for AppError {
    fn from(error: Box<dyn StdError>) -> Self {
        Self::Config(error)
    }
}

impl From<sled::Error> for AppError {
    fn from(error: sled::Error) -> Self {
        Self::Storage(error)
    }
}

impl From<UrlServiceError> for AppError {
    fn from(error: UrlServiceError) -> Self {
        Self::Service(error)
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::Server(error)
    }
}

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn StdError>> {
    // Initialize configuration first
    let config = Arc::new(Config::new().map_err(AppError::Config)?);
    
    // Initialize tracing with config
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(&config.logging.level)))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting application with log level: {}", config.logging.level);
    info!("Initializing storage...");
    let db = sled::Config::new()
        .path(&config.storage.path)
        .mode(sled::Mode::HighThroughput)
        .flush_every_ms(Some(config.storage.flush_interval_ms))
        .cache_capacity((1024 * 1024 * config.storage.cache_size_mb) as u64)
        .open()
        .map_err(AppError::Storage)?;

    // Initialize repository and service
    let repository = Arc::new(SledUrlRepository::new(db));
    debug!("Initializing URL service with encryption...");
    let service = match UrlService::new(repository, Arc::clone(&config)) {
        Ok(service) => {
            info!("URL service initialized successfully");
            web::Data::new(service)
        },
        Err(e) => {
            error!("Failed to initialize URL service: {}", e);
            return Err(AppError::Service(e).into());
        }
    };
    
    info!("Starting server at {}", config.server_url());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["Content-Type"])
                    .max_age(3600)
            )
            .app_data(service.clone())
            // Root level redirect for short codes
            .route("/{short_url}", web::get().to(redirect))
            .service(
                web::scope("/api/v1")
                    .route("/health", web::get().to(health_check))
                    .route("/shorten", web::post().to(shorten_url))
            )
    })
    .bind(config.socket_addr())
    .map_err(AppError::Server)?
    .workers(config.server.workers)
    .run()
    .await
    .map_err(AppError::Server)?;

    Ok(())
}
