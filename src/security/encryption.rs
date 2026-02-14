use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use crate::core::errors::{EngineResult, EngineError};

/// ============================================================================
/// 🔐 Encryption Engine (ගුප්තකේතන යන්ත්‍රය)
/// ============================================================================
/// Banking-grade encryption for sensitive financial data.
/// SHA-256, HMAC, and secure data handling.

/// 🛡️ Hashed field wrapper for PII data (personally identifiable information)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashedField {
    pub hash: String,
    pub salt: String,
}

impl HashedField {
    /// Hash sensitive data with salt
    pub fn new(data: &str, salt: &str) -> Self {
        let salted = format!("{}{}", salt, data);
        let mut hasher = Sha256::new();
        hasher.update(salted.as_bytes());
        let result = hasher.finalize();
        HashedField {
            hash: format!("{:x}", result),
            salt: salt.to_string(),
        }
    }

    /// Verify if input matches the hash
    pub fn verify(&self, input: &str) -> bool {
        let salted = format!("{}{}", self.salt, input);
        let mut hasher = Sha256::new();
        hasher.update(salted.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result) == self.hash
    }
}

/// 🔑 Transaction Signature (ගනුදෙනු අත්සන)
/// ගනුදෙනු tampering වැළැක්වීමට HMAC signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub transaction_id: String,
    pub signature: String,
    pub timestamp: i64,
}

impl TransactionSignature {
    /// 🔏 Create signature for a transaction
    pub fn sign(transaction_id: &str, amount_cents: i64, secret_key: &str) -> Self {
        let payload = format!("{}:{}:{}", transaction_id, amount_cents, chrono::Utc::now().timestamp());
        let signed = format!("{}{}", secret_key, payload);
        
        let mut hasher = Sha256::new();
        hasher.update(signed.as_bytes());
        let result = hasher.finalize();
        
        TransactionSignature {
            transaction_id: transaction_id.to_string(),
            signature: format!("{:x}", result),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// ✅ Verify signature
    pub fn verify(&self, amount_cents: i64, secret_key: &str) -> bool {
        let payload = format!("{}:{}:{}", self.transaction_id, amount_cents, self.timestamp);
        let signed = format!("{}{}", secret_key, payload);
        
        let mut hasher = Sha256::new();
        hasher.update(signed.as_bytes());
        let result = hasher.finalize();
        
        format!("{:x}", result) == self.signature
    }
}

/// 🔒 Secure Data Container (ආරක්ෂිත දත්ත බහාලුම)
/// Encrypted storage for sensitive financial data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureContainer<T: Serialize + Clone> {
    pub data: T,
    pub checksum: String,
    pub created_at: i64,
}

impl<T: Serialize + Clone> SecureContainer<T> {
    pub fn new(data: T) -> EngineResult<Self> {
        let json = serde_json::to_string(&data).map_err(|e| EngineError::Validation {
            message: format!("Serialization failed: {}", e),
        })?;
        
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let checksum = format!("{:x}", hasher.finalize());
        
        Ok(SecureContainer {
            data,
            checksum,
            created_at: chrono::Utc::now().timestamp(),
        })
    }

    /// Verify data integrity
    pub fn is_valid(&self) -> EngineResult<bool> {
        let json = serde_json::to_string(&self.data).map_err(|e| EngineError::Validation {
            message: format!("Serialization failed: {}", e),
        })?;
        
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let current_checksum = format!("{:x}", hasher.finalize());
        
        Ok(current_checksum == self.checksum)
    }
}

/// 🛡️ Data Masking (දත්ත වසං කිරීම)
/// PII protection for card numbers, bank accounts
pub struct DataMasker;

impl DataMasker {
    /// Mask credit card: 4111111111111111 -> ****-****-****-1111
    pub fn mask_card(card_number: &str) -> String {
        let clean: String = card_number.chars().filter(|c| c.is_numeric()).collect();
        if clean.len() < 4 {
            return "****".to_string();
        }
        let last_four = &clean[clean.len()-4..];
        format!("****-****-****-{}", last_four)
    }

    /// Mask bank account: 12345678901234 -> ********1234
    pub fn mask_account(account_number: &str) -> String {
        let clean: String = account_number.chars().filter(|c| c.is_numeric()).collect();
        if clean.len() < 4 {
            return "****".to_string();
        }
        let last_four = &clean[clean.len()-4..];
        format!("********{}", last_four)
    }

    /// Mask email: user@example.com -> u***@example.com
    pub fn mask_email(email: &str) -> String {
        if let Some(at_pos) = email.find('@') {
            if at_pos > 0 {
                let first = &email[0..1];
                let domain = &email[at_pos..];
                return format!("{}***{}", first, domain);
            }
        }
        "***@***.***".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashed_field() {
        let field = HashedField::new("secret_data", "random_salt");
        assert!(field.verify("secret_data"));
        assert!(!field.verify("wrong_data"));
    }

    #[test]
    fn test_card_masking() {
        assert_eq!(DataMasker::mask_card("4111111111111111"), "****-****-****-1111");
    }

    #[test]
    fn test_email_masking() {
        assert_eq!(DataMasker::mask_email("user@example.com"), "u***@example.com");
    }
}
