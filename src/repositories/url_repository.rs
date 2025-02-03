use sled::Db;
use std::sync::Arc;

#[derive(Debug)]
pub enum RepositoryError {
    Storage(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub struct SledUrlRepository {
    db: Arc<Db>,
}

#[derive(Debug)]
pub struct Entry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl SledUrlRepository {
    pub fn new(db: Db) -> Self {
        Self {
            db: Arc::new(db),
        }
    }

    pub async fn store(&self, short_url: &str, long_url: &str) -> Result<(), RepositoryError> {
        self.db
            .insert(short_url.as_bytes(), long_url.as_bytes())
            .map_err(|e| RepositoryError::Storage(format!("Failed to store URL mapping: {}", e)))?;
        
        self.db
            .flush_async()
            .await
            .map_err(|e| RepositoryError::Storage(format!("Failed to flush database: {}", e)))?;
            
        Ok(())
    }

    pub async fn get(&self, short_url: &str) -> Result<Option<String>, RepositoryError> {
        let result = self.db
            .get(short_url.as_bytes())
            .map_err(|e| RepositoryError::Storage(format!("Failed to retrieve URL mapping: {}", e)))?;
            
        Ok(result
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned()))
    }

    pub async fn scan_prefix(&self, prefix: &str) -> Result<impl Iterator<Item = Result<Entry, RepositoryError>>, RepositoryError> {
        let iter = self.db
            .scan_prefix(prefix.as_bytes())
            .map(|res| {
                res.map(|(key, value)| Entry {
                    key: key.to_vec(),
                    value: value.to_vec(),
                })
                .map_err(|e| RepositoryError::Storage(format!("Failed to scan database: {}", e)))
            });
        
        Ok(iter)
    }
} 