//! # 👤 Centralized Accounting (Debtor/Creditor)
//! Manages financial identities for ALL users across ALL engines.

use crate::ledger::account::AccountType;
use crate::ledger::engine::LedgerEngine;
use anyhow::Result;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;
// sqlx::Row removed
// Duplicate import removed
// Wait, warning says 'unused import: sqlx::Row'.
// Line 10: "use sqlx::Row;"
// Line 4: "use sqlx::PgPool;"
// "sqlx::query" uses implicitly.
// I will just remove it.

pub struct AccountManager {
    ledger: LedgerEngine,
}

impl AccountManager {
    pub fn new() -> Self {
        AccountManager {
            ledger: LedgerEngine::new(),
        }
    }

    /// Create a financial identity for a NEW entity (User, Rider, Shop)
    /// This auto-creates a Sub-Ledger account for them.
    pub async fn create_entity_account(
        &self,
        pool: &PgPool,
        entity_id: String,
        entity_type: &str, // "rider", "user", "supplier"
        name: String,
    ) -> Result<Uuid> {
        let _account_type = match entity_type {
            "supplier" => AccountType::Liability, // We owe them money (Payable)
            "user" | "rider" => AccountType::Asset, // They hold money in our wallet (Liability from our perspective, but Asset grouping for wallet usually Liability too? Let's assume Liability: Wallet Deposit)
            // Wait, Users' Wallet Balance is a LIABILITY to the Platform.
            // Platform Cash is an ASSET.
            // So User Account = LIABILITY.
            _ => AccountType::Liability,
        };

        let acc_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO accounts (id, name, account_type, code, restricted, currency, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(acc_id)
        .bind(name)
        .bind("liability")
        .bind(entity_id)
        .bind(false)
        .bind("LKR")
        .bind(chrono::Utc::now())
        .execute(pool)
        .await?;

        Ok(acc_id)
    }

    /// Check Balance (Live from Ledger)
    pub async fn get_balance(&self, pool: &PgPool, entity_id: &str) -> Result<Decimal> {
        // 1. Find Account ID by Entity ID (Code)
        let rec: Option<Uuid> = sqlx::query_scalar("SELECT id FROM accounts WHERE code = $1")
            .bind(entity_id)
            .fetch_optional(pool)
            .await?;

        if let Some(id) = rec {
            // 2. Sum up Journal Entries
            self.ledger.get_balance(pool, id).await
        } else {
            Ok(Decimal::ZERO) // No account = 0 balance
        }
    }
}
