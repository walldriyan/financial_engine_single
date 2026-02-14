use crate::core::money::Money;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// ============================================================================
/// 🗄️ Database Models (දත්ත ආකෘති)
/// ============================================================================
/// මෙහි සියලුම ORM Models අර්ථ දක්වනු ලැබේ.
/// PostgreSQL සහ අනෙකුත් DB සඳහා පොදු ආකෘති.

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionRecord {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub total_amount: i64, // Stored in cents
    pub tax_amount: i64,
    pub currency: String,
    pub status: String,
}

// TODO: Add more models here as the schema evolves
