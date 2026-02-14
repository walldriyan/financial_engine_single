use crate::core::errors::{EngineError, EngineResult};
use crate::storage::config::{MultiDbConfig, StorageMode};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::OnceLock;

/// ============================================================================
/// 🔌 Universal Database Connector (විශ්වීය දත්ත සම්බන්ධකය)
/// ============================================================================
/// PostgreSQL, Supabase, Local DB සියල්ල එකම තැනකින් සම්බන්ධ වේ.
/// Hybrid Mode සඳහා ද සහාය දක්වයි.

pub struct GlobalDb {
    pub sql_pool: Option<Pool<Postgres>>,
    // pub nosql_client: Option<Client>, // e.g. Mongo/Firebase
    pub config: MultiDbConfig,
}

impl GlobalDb {
    /// 🚀 Initialize Connection (සම්බන්ධතාවය ආරම්භ කරන්න)
    pub async fn init(config: MultiDbConfig) -> EngineResult<Self> {
        let mut sql_pool = None;

        // 1. Connect to SQL (if needed)
        match config.mode {
            StorageMode::SqlOnly | StorageMode::Hybrid => {
                println!("🔌 Connecting to SQL Database...");
                let pool = PgPoolOptions::new()
                    .max_connections(config.sql_max_connections)
                    .acquire_timeout(std::time::Duration::from_secs(30))
                    .connect(&config.sql_url)
                    .await
                    .map_err(|e| EngineError::Database {
                        message: format!("SQL Connection Failed: {}", e),
                    })?;

                println!("✅ Connected to SQL Database.");
                sql_pool = Some(pool);
            }
            _ => {}
        }

        // 2. Connect to NoSQL (if needed)
        // match config.mode {
        //    StorageMode::NoSqlOnly | StorageMode::Hybrid => {
        //         // Initialize Firebase/Mongo here
        //    }
        //    _ => {}
        // }

        Ok(GlobalDb { sql_pool, config })
    }

    /// 🛡️ Get SQL Pool (Safe Access)
    pub fn get_sql(&self) -> EngineResult<&Pool<Postgres>> {
        self.sql_pool.as_ref().ok_or(EngineError::Database {
            message: "SQL Database is not configured for this mode.".to_string(),
        })
    }
}

/// 🔒 Singleton DB Access
static GLOBAL_DB: OnceLock<GlobalDb> = OnceLock::new();

pub async fn init_db() -> EngineResult<()> {
    // Already initialized?
    if GLOBAL_DB.get().is_some() {
        return Ok(());
    }

    let config = crate::storage::config::get_config().clone();
    let db = GlobalDb::init(config).await?;

    // Set global instance
    let _ = GLOBAL_DB.set(db);
    Ok(())
}

pub fn get_db() -> EngineResult<&'static GlobalDb> {
    GLOBAL_DB.get().ok_or(EngineError::Database {
        message: "Database not initialized. Call init_db() first.".to_string(),
    })
}
