use serde::{Deserialize, Serialize};
use crate::core::errors::{EngineResult, EngineError};
use crate::core::calculation::CalculationResult;

/// ============================================================================
/// 💾 Database Adapter (දත්ත සමුදාය ඇඩැප්ටරය)
/// ============================================================================
/// Universal database interface for:
/// - SQL: PostgreSQL, MySQL, SQLite, SQL Server
/// - NoSQL: MongoDB, Redis, DynamoDB
/// - ORM: Prisma, Diesel, SQLx
/// - JSON file storage

/// 🔌 Storage Backend Trait (ගබඩා පසුබිම)
pub trait StorageBackend: Send + Sync {
    /// Store a value
    fn set(&self, key: &str, value: &str) -> EngineResult<()>;
    
    /// Get a value
    fn get(&self, key: &str) -> EngineResult<Option<String>>;
    
    /// Delete a value
    fn delete(&self, key: &str) -> EngineResult<bool>;
    
    /// Check if key exists
    fn exists(&self, key: &str) -> EngineResult<bool>;
    
    /// List keys with pattern
    fn keys(&self, pattern: &str) -> EngineResult<Vec<String>>;
}

/// 📊 Repository Trait (දත්ත ගබඩාව)
pub trait Repository<T: Serialize + for<'de> Deserialize<'de>> {
    /// Create or insert
    fn create(&self, entity: &T) -> EngineResult<String>;
    
    /// Find by ID
    fn find_by_id(&self, id: &str) -> EngineResult<Option<T>>;
    
    /// Find all (with optional pagination)
    fn find_all(&self, limit: Option<i32>, offset: Option<i32>) -> EngineResult<Vec<T>>;
    
    /// Update
    fn update(&self, id: &str, entity: &T) -> EngineResult<()>;
    
    /// Delete
    fn delete(&self, id: &str) -> EngineResult<bool>;
    
    /// Count
    fn count(&self) -> EngineResult<i64>;
}

/// 📁 JSON File Storage (JSON ගොනු ගබඩාව)
/// Development/Testing backend
pub struct JsonFileStorage {
    base_path: String,
}

impl JsonFileStorage {
    pub fn new(base_path: &str) -> Self {
        JsonFileStorage {
            base_path: base_path.to_string(),
        }
    }

    fn get_file_path(&self, key: &str) -> String {
        format!("{}/{}.json", self.base_path, key.replace(":", "_"))
    }
}

impl StorageBackend for JsonFileStorage {
    fn set(&self, key: &str, value: &str) -> EngineResult<()> {
        let path = self.get_file_path(key);
        std::fs::write(&path, value).map_err(|e| EngineError::Storage {
            message: format!("Failed to write file {}: {}", path, e),
        })
    }

    fn get(&self, key: &str) -> EngineResult<Option<String>> {
        let path = self.get_file_path(key);
        match std::fs::read_to_string(&path) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(EngineError::Storage {
                message: format!("Failed to read file {}: {}", path, e),
            }),
        }
    }

    fn delete(&self, key: &str) -> EngineResult<bool> {
        let path = self.get_file_path(key);
        match std::fs::remove_file(&path) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(EngineError::Storage {
                message: format!("Failed to delete file {}: {}", path, e),
            }),
        }
    }

    fn exists(&self, key: &str) -> EngineResult<bool> {
        let path = self.get_file_path(key);
        Ok(std::path::Path::new(&path).exists())
    }

    fn keys(&self, pattern: &str) -> EngineResult<Vec<String>> {
        let entries = std::fs::read_dir(&self.base_path).map_err(|e| EngineError::Storage {
            message: format!("Failed to read directory: {}", e),
        })?;

        let mut keys = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".json") {
                    let key = name.replace(".json", "").replace("_", ":");
                    if pattern == "*" || key.contains(pattern) {
                        keys.push(key);
                    }
                }
            }
        }
        Ok(keys)
    }
}

/// 🧠 In-Memory Storage (මතක ගබඩාව)
/// Fast caching and testing
pub struct InMemoryStorage {
    data: std::sync::RwLock<std::collections::HashMap<String, String>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage {
            data: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

impl StorageBackend for InMemoryStorage {
    fn set(&self, key: &str, value: &str) -> EngineResult<()> {
        let mut data = self.data.write().map_err(|_| EngineError::Storage {
            message: "Lock poisoned".to_string(),
        })?;
        data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn get(&self, key: &str) -> EngineResult<Option<String>> {
        let data = self.data.read().map_err(|_| EngineError::Storage {
            message: "Lock poisoned".to_string(),
        })?;
        Ok(data.get(key).cloned())
    }

    fn delete(&self, key: &str) -> EngineResult<bool> {
        let mut data = self.data.write().map_err(|_| EngineError::Storage {
            message: "Lock poisoned".to_string(),
        })?;
        Ok(data.remove(key).is_some())
    }

    fn exists(&self, key: &str) -> EngineResult<bool> {
        let data = self.data.read().map_err(|_| EngineError::Storage {
            message: "Lock poisoned".to_string(),
        })?;
        Ok(data.contains_key(key))
    }

    fn keys(&self, pattern: &str) -> EngineResult<Vec<String>> {
        let data = self.data.read().map_err(|_| EngineError::Storage {
            message: "Lock poisoned".to_string(),
        })?;
        Ok(data.keys()
            .filter(|k| pattern == "*" || k.contains(pattern))
            .cloned()
            .collect())
    }
}

/// 📝 Entity Serializer (object -> JSON)
pub struct EntitySerializer;

impl EntitySerializer {
    /// Serialize to JSON string
    pub fn to_json<T: Serialize>(entity: &T) -> EngineResult<String> {
        serde_json::to_string(entity).map_err(|e| EngineError::Storage {
            message: format!("Serialization failed: {}", e),
        })
    }

    /// Serialize to pretty JSON
    pub fn to_json_pretty<T: Serialize>(entity: &T) -> EngineResult<String> {
        serde_json::to_string_pretty(entity).map_err(|e| EngineError::Storage {
            message: format!("Serialization failed: {}", e),
        })
    }

    /// Deserialize from JSON
    pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> EngineResult<T> {
        serde_json::from_str(json).map_err(|e| EngineError::Storage {
            message: format!("Deserialization failed: {}", e),
        })
    }

    /// Serialize calculation result for API/DB
    pub fn serialize_calculation(result: &CalculationResult) -> EngineResult<String> {
        let dto = serde_json::json!({
            "subtotal": result.subtotal.amount,
            "discount_total": result.discount_total.amount,
            "tax_total": result.tax_total.amount,
            "grand_total": result.grand_total.amount,
            "subtotal_formatted": result.subtotal.to_string(),
            "discount_formatted": result.discount_total.to_string(),
            "tax_formatted": result.tax_total.to_string(),
            "total_formatted": result.grand_total.to_string(),
        });
        serde_json::to_string(&dto).map_err(|e| EngineError::Storage {
            message: format!("Serialization failed: {}", e),
        })
    }
}

/// 🔗 Database Connection Config (දත්ත සමුදා සම්බන්ධතා වින්‍යාසය)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub driver: DatabaseDriver,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssl: bool,
    pub pool_size: u32,
    pub connection_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseDriver {
    PostgreSQL,
    MySQL,
    SQLite,
    SQLServer,
    MongoDB,
    Redis,
    DynamoDB,
    InMemory,
    JsonFile,
}

impl DatabaseConfig {
    /// Create connection string for SQL databases
    pub fn connection_string(&self) -> String {
        match self.driver {
            DatabaseDriver::PostgreSQL => {
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    self.username.as_deref().unwrap_or("postgres"),
                    self.password.as_deref().unwrap_or(""),
                    self.host,
                    self.port,
                    self.database
                )
            }
            DatabaseDriver::MySQL => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username.as_deref().unwrap_or("root"),
                    self.password.as_deref().unwrap_or(""),
                    self.host,
                    self.port,
                    self.database
                )
            }
            DatabaseDriver::SQLite => {
                format!("sqlite://{}", self.database)
            }
            DatabaseDriver::MongoDB => {
                format!(
                    "mongodb://{}:{}@{}:{}/{}",
                    self.username.as_deref().unwrap_or("admin"),
                    self.password.as_deref().unwrap_or(""),
                    self.host,
                    self.port,
                    self.database
                )
            }
            _ => String::new(),
        }
    }

    /// Default PostgreSQL config
    pub fn postgres_default() -> Self {
        DatabaseConfig {
            driver: DatabaseDriver::PostgreSQL,
            host: "localhost".to_string(),
            port: 5432,
            database: "financial_engine".to_string(),
            username: Some("postgres".to_string()),
            password: None,
            ssl: false,
            pool_size: 10,
            connection_timeout_ms: 5000,
        }
    }

    /// Default In-Memory config (testing)
    pub fn in_memory() -> Self {
        DatabaseConfig {
            driver: DatabaseDriver::InMemory,
            host: String::new(),
            port: 0,
            database: String::new(),
            username: None,
            password: None,
            ssl: false,
            pool_size: 1,
            connection_timeout_ms: 0,
        }
    }
}

/// 📋 SQL Schema Generator (SQL ක්‍රමය උත්පාදකය)
pub struct SchemaGenerator;

impl SchemaGenerator {
    /// Generate PostgreSQL schema
    pub fn postgres_schema() -> &'static str {
        r#"
        -- Financial Engine Database Schema
        
        CREATE TABLE IF NOT EXISTS transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            transaction_type VARCHAR(50) NOT NULL,
            subtotal BIGINT NOT NULL,
            discount_total BIGINT NOT NULL,
            tax_total BIGINT NOT NULL,
            grand_total BIGINT NOT NULL,
            currency VARCHAR(3) DEFAULT 'LKR',
            customer_id UUID,
            status VARCHAR(20) DEFAULT 'pending',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            metadata JSONB
        );

        CREATE TABLE IF NOT EXISTS transaction_items (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            transaction_id UUID REFERENCES transactions(id),
            item_id VARCHAR(100) NOT NULL,
            item_name VARCHAR(255) NOT NULL,
            unit_price BIGINT NOT NULL,
            quantity DECIMAL(10,4) NOT NULL,
            discount BIGINT DEFAULT 0,
            tax BIGINT DEFAULT 0,
            total BIGINT NOT NULL,
            metadata JSONB
        );

        CREATE TABLE IF NOT EXISTS ledger_entries (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            transaction_id UUID REFERENCES transactions(id),
            account_id VARCHAR(50) NOT NULL,
            debit BIGINT DEFAULT 0,
            credit BIGINT DEFAULT 0,
            description TEXT,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS audit_log (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            action VARCHAR(50) NOT NULL,
            severity VARCHAR(20) NOT NULL,
            resource_type VARCHAR(50) NOT NULL,
            resource_id VARCHAR(100),
            user_id VARCHAR(100),
            ip_address VARCHAR(45),
            old_value JSONB,
            new_value JSONB,
            description TEXT,
            checksum VARCHAR(64),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS inventory_stock (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            warehouse_id VARCHAR(50) NOT NULL,
            item_id VARCHAR(100) NOT NULL,
            quantity DECIMAL(10,4) NOT NULL DEFAULT 0,
            min_quantity DECIMAL(10,4) DEFAULT 0,
            max_quantity DECIMAL(10,4),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            UNIQUE(warehouse_id, item_id)
        );

        CREATE INDEX idx_transactions_customer ON transactions(customer_id);
        CREATE INDEX idx_transactions_created ON transactions(created_at);
        CREATE INDEX idx_ledger_account ON ledger_entries(account_id);
        CREATE INDEX idx_audit_action ON audit_log(action);
        CREATE INDEX idx_audit_user ON audit_log(user_id);
        "#
    }

    /// Generate Prisma schema
    pub fn prisma_schema() -> &'static str {
        r#"
        // Prisma Schema for Financial Engine
        
        generator client {
          provider = "prisma-client-js"
        }

        datasource db {
          provider = "postgresql"
          url      = env("DATABASE_URL")
        }

        model Transaction {
          id            String   @id @default(uuid())
          type          String
          subtotal      BigInt
          discountTotal BigInt
          taxTotal      BigInt
          grandTotal    BigInt
          currency      String   @default("LKR")
          customerId    String?
          status        String   @default("pending")
          createdAt     DateTime @default(now())
          updatedAt     DateTime @updatedAt
          items         TransactionItem[]
          ledgerEntries LedgerEntry[]
        }

        model TransactionItem {
          id            String      @id @default(uuid())
          transactionId String
          transaction   Transaction @relation(fields: [transactionId], references: [id])
          itemId        String
          itemName      String
          unitPrice     BigInt
          quantity      Decimal
          discount      BigInt      @default(0)
          tax           BigInt      @default(0)
          total         BigInt
        }

        model LedgerEntry {
          id            String      @id @default(uuid())
          transactionId String
          transaction   Transaction @relation(fields: [transactionId], references: [id])
          accountId     String
          debit         BigInt      @default(0)
          credit        BigInt      @default(0)
          description   String?
          createdAt     DateTime    @default(now())
        }

        model AuditLog {
          id           String   @id @default(uuid())
          action       String
          severity     String
          resourceType String
          resourceId   String?
          userId       String?
          ipAddress    String?
          description  String?
          checksum     String
          createdAt    DateTime @default(now())
        }
        "#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_storage() {
        let storage = InMemoryStorage::new();
        storage.set("test:key", "test_value").unwrap();
        
        let value = storage.get("test:key").unwrap();
        assert_eq!(value, Some("test_value".to_string()));
        
        storage.delete("test:key").unwrap();
        assert!(!storage.exists("test:key").unwrap());
    }

    #[test]
    fn test_entity_serialization() {
        let money = Money::new(100, 50);
        let json = EntitySerializer::to_json(&money).unwrap();
        assert!(json.contains("10050"));
    }

    #[test]
    fn test_connection_string() {
        let config = DatabaseConfig::postgres_default();
        let conn = config.connection_string();
        assert!(conn.starts_with("postgres://"));
    }
}
