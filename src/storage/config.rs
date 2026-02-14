use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

/// ============================================================================
/// 🗄️ Universal Storage Configuration (විශ්වීය දත්ත ගබඩා සැකසුම්)
/// ============================================================================
/// SQL (Postgres/MySQL) සහ NoSQL (Firebase/Mongo) එකම තැනකින් පාලනය කරයි.
/// Hybrid Mode මගින් දෙකම එකවර භාවිතා කළ හැක.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageMode {
    SqlOnly,   // Traditional Banking (ACID)
    NoSqlOnly, // Modern App / Realtime
    Hybrid,    // SQL for Ledger, NoSQL for App State (Best of both worlds)
    InMemory,  // Testing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiDbConfig {
    pub mode: StorageMode,

    // 🐘 SQL Configuration (PostgreSQL / SQLite)
    pub sql_url: String,
    pub sql_max_connections: u32,

    // 🔥 NoSQL Configuration (Firebase / MongoDB / DynamoDB)
    pub nosql_url: Option<String>,
    pub firebase_project_id: Option<String>,
    pub firebase_api_key: Option<String>,

    // 🚀 Cache Configuration (Redis)
    pub redis_url: Option<String>,

    // 🛡️ Error Tracking (Sentry)
    pub sentry_dsn: Option<String>,
}

impl MultiDbConfig {
    /// 🚀 Load from Environment (Auto-detects settings)
    pub fn from_env() -> Self {
        // Mode Detection
        let mode_str = env::var("STORAGE_MODE").unwrap_or_else(|_| "sql_only".to_string());
        let mode = match mode_str.to_lowercase().as_str() {
            "nosql" => StorageMode::NoSqlOnly,
            "hybrid" => StorageMode::Hybrid,
            "memory" => StorageMode::InMemory,
            _ => StorageMode::SqlOnly,
        };

        MultiDbConfig {
            mode,

            // SQL Defaults
            sql_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://user:pass@localhost:5432/financial_engine".to_string()
            }),
            sql_max_connections: env::var("DB_POOL_SIZE")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .unwrap_or(50),

            // NoSQL Defaults
            nosql_url: env::var("NOSQL_URL").ok(),
            firebase_project_id: env::var("FIREBASE_PROJECT_ID").ok(),
            firebase_api_key: env::var("FIREBASE_API_KEY").ok(),

            // Redis Defaults
            redis_url: env::var("REDIS_URL").ok(),

            // Sentry Defaults
            sentry_dsn: env::var("SENTRY_DSN").ok(),
        }
    }
}

/// 🔒 Singleton Config Instance
pub fn get_config() -> &'static MultiDbConfig {
    static CONFIG: OnceLock<MultiDbConfig> = OnceLock::new();
    CONFIG.get_or_init(|| MultiDbConfig::from_env())
}
