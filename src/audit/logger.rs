use crate::core::errors::EngineResult;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Mutex;

/// ============================================================================
/// 📊 Centralized Audit Logger (මධ්‍යගත විගණන සටහන්)
/// ============================================================================
/// පද්ධතියේ සිදුවන සියලුම දේ මෙහි සටහන් වේ.
/// Debug logs, Errors, සහ Transaction History සියල්ල එකම තැනක.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Audit, // Banking grade audit record
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub module: String,
    pub action: String,
    pub details: String,
    pub previous_hash: String, // 🔗 Chain Link
    pub hash: String,          // 🔒 Current Hash (SHA-256)
}

lazy_static! {
    static ref LAST_HASH: Mutex<String> = Mutex::new("GENESIS_HASH".to_string());
}

pub struct Logger {
    // In a real implementation, this might hold database connections or file handles
}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }

    /// 📝 සටහන් තබන්න (Log Record with Hash Chain)
    pub fn log(
        &self,
        level: LogLevel,
        module: &str,
        action: &str,
        details: &str,
    ) -> EngineResult<()> {
        let mut last_hash_lock = LAST_HASH.lock().unwrap();
        let prev_hash = last_hash_lock.clone();

        let timestamp = Utc::now();
        let id = uuid::Uuid::new_v4().to_string();

        // Calculate Hash: SHA256(prev_hash + id + timestamp + level + module + action + details)
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{:?}{}{}{}",
            prev_hash, id, timestamp, level, module, action, details
        ));
        let current_hash = format!("{:x}", hasher.finalize());

        // Update global state
        *last_hash_lock = current_hash.clone();

        let entry = LogEntry {
            id,
            timestamp,
            level: level.clone(),
            module: module.to_string(),
            action: action.to_string(),
            details: details.to_string(),
            previous_hash: prev_hash,
            hash: current_hash,
        };

        // For now, just print to stdout. In production, this goes to DB/File.
        println!(
            "[{}] [{:?}] {}: {} - {} [Hash: {}]",
            entry.timestamp, entry.level, entry.module, entry.action, entry.details, entry.hash
        );

        Ok(())
    }

    /// 🚨 දෝෂ සටහන් තබන්න (Error Log with Source Tracking)
    pub fn log_error(
        &self,
        module: &str,
        error_msg: &str,
        file: &str,
        line: u32,
    ) -> EngineResult<()> {
        let details = format!("ERROR at {}:{} -> {}", file, line, error_msg);
        self.log(LogLevel::Error, module, "EXCEPTION", &details)
    }
}
