use std::sync::Arc;
use std::fmt;
use url::Url;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use tracing::debug;
use crate::{
    config::Config,
    repositories::url_repository::{SledUrlRepository, RepositoryError},
    services::encryption_service::{EncryptionService, EncryptionError},
};

#[derive(Debug)]
pub enum UrlServiceError {
    InvalidUrl(String),
    NotFound(String),
    StorageError(RepositoryError),
    InvalidAlias(String),
    AliasExists(String),
    EncryptionError(EncryptionError),
}

impl std::fmt::Display for UrlServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl(url) => write!(f, "Invalid URL format: {}", url),
            Self::NotFound(code) => write!(f, "URL not found: {}", code),
            Self::StorageError(e) => write!(f, "Storage error: {}", e),
            Self::InvalidAlias(msg) => write!(f, "Invalid alias: {}", msg),
            Self::AliasExists(alias) => write!(f, "Alias already exists: {}", alias),
            Self::EncryptionError(e) => write!(f, "Encryption error: {}", e),
        }
    }
}

impl std::error::Error for UrlServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::StorageError(e) => Some(e),
            Self::EncryptionError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<RepositoryError> for UrlServiceError {
    fn from(error: RepositoryError) -> Self {
        UrlServiceError::StorageError(error)
    }
}

impl From<EncryptionError> for UrlServiceError {
    fn from(error: EncryptionError) -> Self {
        UrlServiceError::EncryptionError(error)
    }
}

#[derive(Debug, Clone)]
pub struct ShortenedUrl {
    pub short_code: String,
    pub original_url: String,
    pub full_short_url: String,
}

pub struct UrlService {
    repository: Arc<SledUrlRepository>,
    config: Arc<Config>,
    encryption: Arc<EncryptionService>,
}

impl UrlService {
    pub fn new(repository: Arc<SledUrlRepository>, config: Arc<Config>) -> Result<Self, UrlServiceError> {
        let encryption = EncryptionService::new()
            .map_err(UrlServiceError::EncryptionError)?;
        
        Ok(Self {
            repository,
            config,
            encryption: Arc::new(encryption),
        })
    }

    fn validate_alias(alias: &str) -> Result<(), UrlServiceError> {
        if alias.len() < 3 {
            return Err(UrlServiceError::InvalidAlias("Alias must be at least 3 characters long".to_string()));
        }
        if alias.len() > 32 {
            return Err(UrlServiceError::InvalidAlias("Alias must not exceed 32 characters".to_string()));
        }
        if !alias.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(UrlServiceError::InvalidAlias("Alias can only contain alphanumeric characters, hyphens, and underscores".to_string()));
        }
        Ok(())
    }

    async fn find_existing_url(&self, url: &str) -> Result<Option<String>, UrlServiceError> {
        let encrypted_url = self.encryption.encrypt(url)?;
        let encoded_url = STANDARD.encode(&encrypted_url);
        
        // Scan through existing entries to find a match
        let iter = self.repository.scan_prefix("").await?;
        for entry in iter {
            let entry = entry?;
            if entry.value == encoded_url.as_bytes() {
                return Ok(Some(String::from_utf8_lossy(&entry.key).to_string()));
            }
        }
        
        Ok(None)
    }

    pub async fn shorten_url(&self, url: String, custom_alias: Option<String>) -> Result<ShortenedUrl, UrlServiceError> {
        // Validate URL format
        let parsed_url = Url::parse(&url)
            .map_err(|_| UrlServiceError::InvalidUrl(url.clone()))?;
        let normalized_url = parsed_url.to_string();

        // If a custom alias is provided, validate and use it
        if let Some(alias) = custom_alias.as_ref() {
            Self::validate_alias(alias)?;
            
            // Check if alias is already taken
            if let Some(existing_url) = self.repository.get(alias).await? {
                let encrypted_data = STANDARD.decode(&existing_url)
                    .map_err(|_| UrlServiceError::StorageError(RepositoryError::Storage(
                        "Invalid base64 data".to_string()
                    )))?;
                
                let decrypted_url = self.encryption.decrypt(&encrypted_data)?;
                
                // If the alias points to the same URL, return it
                if decrypted_url == normalized_url {
                    debug!("URL already exists with requested alias: {}", alias);
                    return Ok(ShortenedUrl {
                        short_code: alias.clone(),
                        original_url: normalized_url,
                        full_short_url: format!("{}/{}", self.config.server_url(), alias),
                    });
                }
                
                return Err(UrlServiceError::AliasExists(alias.clone()));
            }
        }

        // Check if URL already exists
        if let Some(existing_code) = self.find_existing_url(&normalized_url).await? {
            debug!("URL already exists with code: {}", existing_code);
            return Ok(ShortenedUrl {
                short_code: existing_code.clone(),
                original_url: normalized_url,
                full_short_url: format!("{}/{}", self.config.server_url(), existing_code),
            });
        }

        // Generate new short code if no custom alias or URL doesn't exist
        let short_code = custom_alias.unwrap_or_else(|| nanoid::nanoid!(6));
        
        // Encrypt URL before storing
        let encrypted_url = self.encryption.encrypt(&normalized_url)?;
        
        // Store encrypted URL
        self.repository
            .store(&short_code, &STANDARD.encode(&encrypted_url))
            .await?;

        Ok(ShortenedUrl {
            short_code: short_code.clone(),
            original_url: normalized_url,
            full_short_url: format!("{}/{}", self.config.server_url(), short_code),
        })
    }

    pub async fn get_url(&self, short_code: &str) -> Result<String, UrlServiceError> {
        match self.repository.get(short_code).await {
            Ok(Some(encrypted_url)) => {
                let encrypted_data = STANDARD.decode(&encrypted_url)
                    .map_err(|_| UrlServiceError::StorageError(RepositoryError::Storage(
                        "Invalid base64 data".to_string()
                    )))?;
                
                Ok(self.encryption.decrypt(&encrypted_data)?)
            }
            Ok(None) => Err(UrlServiceError::NotFound(short_code.to_string())),
            Err(e) => Err(UrlServiceError::StorageError(e)),
        }
    }
} 