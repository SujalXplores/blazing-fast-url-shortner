use std::fs;
use std::path::Path;
use ring::{aead::{self, LessSafeKey, UnboundKey, AES_256_GCM, Nonce, NONCE_LEN}, rand::{SecureRandom, SystemRandom}};
use base64::{engine::general_purpose::STANDARD, Engine};

const KEY_FILE_PATH: &str = "encryption.key";

#[derive(Debug)]
pub enum EncryptionError {
    KeyGeneration(String),
    KeyStorage(String),
    Encryption(String),
    Decryption(String),
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyGeneration(msg) => write!(f, "Key generation error: {}", msg),
            Self::KeyStorage(msg) => write!(f, "Key storage error: {}", msg),
            Self::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            Self::Decryption(msg) => write!(f, "Decryption error: {}", msg),
        }
    }
}

impl std::error::Error for EncryptionError {}

pub struct EncryptionService {
    key: LessSafeKey,
    rng: SystemRandom,
}

impl EncryptionService {
    pub fn new() -> Result<Self, EncryptionError> {
        let rng = SystemRandom::new();
        let key_bytes = Self::load_or_generate_key(&rng)?;
        
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
            .map_err(|_| EncryptionError::KeyGeneration("Failed to create encryption key".to_string()))?;
        
        Ok(Self {
            key: LessSafeKey::new(unbound_key),
            rng,
        })
    }

    fn load_or_generate_key(rng: &SystemRandom) -> Result<[u8; 32], EncryptionError> {
        if Path::new(KEY_FILE_PATH).exists() {
            // Load existing key
            let encoded_key = fs::read_to_string(KEY_FILE_PATH)
                .map_err(|e| EncryptionError::KeyStorage(format!("Failed to read key file: {}", e)))?;
            
            let key_bytes = STANDARD.decode(encoded_key)
                .map_err(|e| EncryptionError::KeyStorage(format!("Failed to decode key: {}", e)))?;
            
            if key_bytes.len() != 32 {
                return Err(EncryptionError::KeyStorage("Invalid key length".to_string()));
            }
            
            let mut key = [0u8; 32];
            key.copy_from_slice(&key_bytes);
            Ok(key)
        } else {
            // Generate new key
            let mut key = [0u8; 32];
            rng.fill(&mut key)
                .map_err(|_| EncryptionError::KeyGeneration("Failed to generate encryption key".to_string()))?;
            
            // Store the key
            let encoded_key = STANDARD.encode(key);
            fs::write(KEY_FILE_PATH, encoded_key)
                .map_err(|e| EncryptionError::KeyStorage(format!("Failed to write key file: {}", e)))?;
            
            Ok(key)
        }
    }

    pub fn encrypt(&self, data: &str) -> Result<Vec<u8>, EncryptionError> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|_| EncryptionError::Encryption("Failed to generate nonce".to_string()))?;
        
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let mut in_out = data.as_bytes().to_vec();
        
        self.key
            .seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| EncryptionError::Encryption("Failed to encrypt data".to_string()))?;

        // Prepend nonce to encrypted data
        let mut result = Vec::with_capacity(NONCE_LEN + in_out.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&in_out);
        
        Ok(result)
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<String, EncryptionError> {
        if encrypted_data.len() < NONCE_LEN {
            return Err(EncryptionError::Decryption("Invalid encrypted data".to_string()));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_LEN);
        let nonce = Nonce::assume_unique_for_key(nonce_bytes.try_into().unwrap());
        
        let mut buffer = ciphertext.to_vec();
        let plaintext = self.key
            .open_in_place(nonce, aead::Aad::empty(), &mut buffer)
            .map_err(|_| EncryptionError::Decryption("Failed to decrypt data".to_string()))?;

        String::from_utf8(plaintext.to_vec())
            .map_err(|_| EncryptionError::Decryption("Invalid UTF-8 in decrypted data".to_string()))
    }
} 