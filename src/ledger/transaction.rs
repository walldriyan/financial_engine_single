use crate::core::money::Money;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ============================================================================
/// 📝 Ledger Transaction (ගනුදෙනු සටහන)
/// ============================================================================
/// Double Entry Principle: Total Debits MUST EQUAL Total Credits.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub account_id: String,
    pub debit: Money,
    pub credit: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub entries: Vec<Entry>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl Transaction {
    pub fn new(description: &str) -> Self {
        Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            date: Utc::now(),
            description: description.to_string(),
            entries: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add a Debit entry (Receive value)
    pub fn debit(mut self, account_id: &str, amount: Money) -> Self {
        self.entries.push(Entry {
            account_id: account_id.to_string(),
            debit: amount,
            credit: Money::zero(),
        });
        self
    }

    /// Add a Credit entry (Give value)
    pub fn credit(mut self, account_id: &str, amount: Money) -> Self {
        self.entries.push(Entry {
            account_id: account_id.to_string(),
            debit: Money::zero(),
            credit: amount,
        });
        self
    }

    /// Validate if Debit == Credit
    pub fn is_balanced(&self) -> bool {
        let mut total_debit = Money::zero();
        let mut total_credit = Money::zero();

        for entry in &self.entries {
            total_debit = total_debit + entry.debit.clone();
            total_credit = total_credit + entry.credit.clone();
        }

        total_debit == total_credit
    }
}
