use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{error, debug};
use crate::services::url_service::{UrlService, UrlServiceError};

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
    #[serde(default)]
    custom_alias: Option<String>,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    short_code: String,
    original_url: String,
    short_url: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

pub async fn shorten_url(
    service: web::Data<UrlService>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
    debug!("Shortening URL: {}", req.url);
    match service.shorten_url(req.url.clone(), req.custom_alias.clone()).await {
        Ok(shortened) => {
            debug!("Successfully shortened URL to: {}", shortened.short_code);
            HttpResponse::Ok().json(ShortenResponse {
                short_code: shortened.short_code,
                original_url: shortened.original_url,
                short_url: shortened.full_short_url,
            })
        },
        Err(UrlServiceError::InvalidUrl(_)) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid URL format"
            }))
        }
        Err(UrlServiceError::InvalidAlias(msg)) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Invalid alias: {}", msg)
            }))
        }
        Err(UrlServiceError::AliasExists(alias)) => {
            HttpResponse::Conflict().json(serde_json::json!({
                "error": format!("Alias '{}' is already taken", alias)
            }))
        }
        Err(UrlServiceError::EncryptionError(e)) => {
            error!("Encryption error while shortening URL: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to secure URL data"
            }))
        }
        Err(e) => {
            error!("Failed to shorten URL: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to shorten URL"
            }))
        }
    }
}

pub async fn redirect(
    service: web::Data<UrlService>,
    path: web::Path<String>,
) -> impl Responder {
    let short_url = path.into_inner();
    debug!("Redirecting short URL: {}", short_url);
    
    match service.get_url(&short_url).await {
        Ok(url) => {
            debug!("Successfully resolved URL: {}", short_url);
            HttpResponse::Found()
                .append_header(("Location", url))
                .finish()
        },
        Err(UrlServiceError::NotFound(_)) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "URL not found"
            }))
        }
        Err(UrlServiceError::EncryptionError(e)) => {
            error!("Encryption error while retrieving URL: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to process URL data"
            }))
        }
        Err(e) => {
            error!("Failed to retrieve URL: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve URL"
            }))
        }
    }
} 