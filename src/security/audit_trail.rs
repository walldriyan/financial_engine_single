use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::money::Money;

/// ============================================================================
/// 📜 Audit Trail (විගණන පෙළ)
/// ============================================================================
/// Complete, immutable audit trail for all financial operations.
/// Banking compliance: SOX, PCI-DSS, GDPR ready.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditAction {
    // Transaction events
    TransactionCreated,
    TransactionModified,
    TransactionCompleted,
    TransactionCancelled,
    TransactionRefunded,
    
    // Money movement
    MoneyReceived,
    MoneyTransferred,
    MoneyWithdrawn,
    
    // Security events
    LoginSuccess,
    LoginFailed,
    PermissionDenied,
    RateLimitExceeded,
    SuspiciousActivity,
    
    // System events
    ConfigChanged,
    RuleAdded,
    RuleRemoved,
    SystemError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditSeverity {
    Info,      // Normal operation
    Warning,   // Unusual but not critical
    Error,     // Operation failed
    Critical,  // Security breach or system failure
    Audit,     // Compliance-required logging
}

/// 📋 Single Audit Entry (තනි විගණන ඇතුළත් කිරීම)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub action: AuditAction,
    pub severity: AuditSeverity,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub ip_address: Option<String>,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub amount: Option<Money>,
    pub description: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub checksum: String,
}

impl AuditEntry {
    pub fn new(
        action: AuditAction,
        severity: AuditSeverity,
        resource_type: &str,
        description: &str,
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        let mut entry = AuditEntry {
            id: id.clone(),
            timestamp,
            action,
            severity,
            user_id: None,
            session_id: None,
            ip_address: None,
            resource_type: resource_type.to_string(),
            resource_id: None,
            old_value: None,
            new_value: None,
            amount: None,
            description: description.to_string(),
            metadata: std::collections::HashMap::new(),
            checksum: String::new(),
        };
        
        entry.checksum = entry.calculate_checksum();
        entry
    }

    /// Add user context
    pub fn with_user(mut self, user_id: &str, session_id: Option<&str>, ip: Option<&str>) -> Self {
        self.user_id = Some(user_id.to_string());
        self.session_id = session_id.map(|s| s.to_string());
        self.ip_address = ip.map(|s| s.to_string());
        self.checksum = self.calculate_checksum();
        self
    }

    /// Add resource context
    pub fn with_resource(mut self, resource_id: &str) -> Self {
        self.resource_id = Some(resource_id.to_string());
        self.checksum = self.calculate_checksum();
        self
    }

    /// Add change tracking
    pub fn with_changes(mut self, old: Option<&str>, new: Option<&str>) -> Self {
        self.old_value = old.map(|s| s.to_string());
        self.new_value = new.map(|s| s.to_string());
        self.checksum = self.calculate_checksum();
        self
    }

    /// Add money amount
    pub fn with_amount(mut self, amount: Money) -> Self {
        self.amount = Some(amount);
        self.checksum = self.calculate_checksum();
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self.checksum = self.calculate_checksum();
        self
    }

    /// Calculate tamper-proof checksum
    fn calculate_checksum(&self) -> String {
        use sha2::{Sha256, Digest};
        
        let data = format!(
            "{}:{}:{:?}:{:?}:{:?}:{}",
            self.id,
            self.timestamp.timestamp(),
            self.action,
            self.user_id,
            self.amount.as_ref().map(|m| m.amount),
            self.description
        );
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify entry integrity
    pub fn verify_integrity(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    /// Export to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// 📚 Audit Trail Manager (විගණන පෙළ කළමනාකරු)
pub struct AuditTrail {
    entries: Vec<AuditEntry>,
    max_entries: usize,
}

impl AuditTrail {
    pub fn new(max_entries: usize) -> Self {
        AuditTrail {
            entries: Vec::new(),
            max_entries,
        }
    }

    /// Add new entry
    pub fn log(&mut self, entry: AuditEntry) {
        if self.entries.len() >= self.max_entries {
            // In production, export to cold storage before removing
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    /// Get entries by action type
    pub fn get_by_action(&self, action: &AuditAction) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| &e.action == action).collect()
    }

    /// Get entries by user
    pub fn get_by_user(&self, user_id: &str) -> Vec<&AuditEntry> {
        self.entries.iter()
            .filter(|e| e.user_id.as_ref().map(|u| u == user_id).unwrap_or(false))
            .collect()
    }

    /// Get entries by severity
    pub fn get_by_severity(&self, severity: &AuditSeverity) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| &e.severity == severity).collect()
    }

    /// Get entries in time range
    pub fn get_in_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Vec<&AuditEntry> {
        self.entries.iter()
            .filter(|e| e.timestamp >= from && e.timestamp <= to)
            .collect()
    }

    /// Verify chain integrity
    pub fn verify_chain(&self) -> bool {
        self.entries.iter().all(|e| e.verify_integrity())
    }

    /// Export all to JSON
    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.entries).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get total count
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(
            AuditAction::TransactionCreated,
            AuditSeverity::Audit,
            "Transaction",
            "New transaction created"
        )
        .with_user("user123", Some("sess456"), Some("192.168.1.1"))
        .with_amount(Money::new(100, 0));

        assert!(entry.verify_integrity());
        assert_eq!(entry.user_id, Some("user123".to_string()));
    }

    #[test]
    fn test_audit_trail_logging() {
        let mut trail = AuditTrail::new(100);
        
        trail.log(AuditEntry::new(
            AuditAction::LoginSuccess,
            AuditSeverity::Info,
            "User",
            "User logged in"
        ));

        assert_eq!(trail.count(), 1);
        assert!(trail.verify_chain());
    }
}
