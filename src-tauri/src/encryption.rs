use crate::errors::AppError;
use base64::{Engine as _, engine::general_purpose};

// Simple encryption for demo purposes
// In production, use proper encryption libraries like aes-gcm or chacha20poly1305

pub struct EncryptionService {
    key: String,
}

impl EncryptionService {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn encrypt(&self, data: &str) -> Result<String, AppError> {
        // Simple XOR encryption for demo
        let key_bytes = self.key.as_bytes();
        let data_bytes = data.as_bytes();
        let mut encrypted = Vec::new();
        
        for (i, &byte) in data_bytes.iter().enumerate() {
            let key_byte = key_bytes[i % key_bytes.len()];
            encrypted.push(byte ^ key_byte);
        }
        
        Ok(general_purpose::STANDARD.encode(encrypted))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String, AppError> {
        let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_data)
            .map_err(|e| AppError::Encryption(e.to_string()))?;
        
        let key_bytes = self.key.as_bytes();
        let mut decrypted = Vec::new();
        
        for (i, &byte) in encrypted_bytes.iter().enumerate() {
            let key_byte = key_bytes[i % key_bytes.len()];
            decrypted.push(byte ^ key_byte);
        }
        
        String::from_utf8(decrypted)
            .map_err(|e| AppError::Encryption(e.to_string()))
    }
}

// Global encryption service instance
lazy_static::lazy_static! {
    pub static ref ENCRYPTION_SERVICE: EncryptionService = {
        EncryptionService::new("yotion-secret-key-2024".to_string())
    };
}

pub fn encrypt_sensitive_data(data: &str) -> Result<String, AppError> {
    ENCRYPTION_SERVICE.encrypt(data)
}

pub fn decrypt_sensitive_data(encrypted_data: &str) -> Result<String, AppError> {
    ENCRYPTION_SERVICE.decrypt(encrypted_data)
}

// Type alias for encrypted data
pub type EncryptedData = String;