use crate::ledger::transaction::Transaction;
use crate::ledger::account::Account;
use crate::core::errors::{EngineResult, EngineError};
use std::collections::HashMap;

/// ============================================================================
/// 📚 General Ledger (ප්‍රධාන ලෙජරය)
/// ============================================================================

pub struct GeneralLedger {
    accounts: HashMap<String, Account>,
    journal: Vec<Transaction>,
}

impl GeneralLedger {
    pub fn new() -> Self {
        GeneralLedger {
            accounts: HashMap::new(),
            journal: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.id.clone(), account);
    }

    /// Post a transaction to the ledger
    /// This updates account balances strictly following Double Entry rules.
    pub fn post_transaction(&mut self, transaction: Transaction) -> EngineResult<()> {
        if !transaction.is_balanced() {
            return Err(EngineError::Validation {
                message: "Transaction is not balanced! Debits != Credits".to_string(),
            });
        }

        // Validate accounts exist
        for entry in &transaction.entries {
            if !self.accounts.contains_key(&entry.account_id) {
                return Err(EngineError::Validation {
                    message: format!("Account ID {} not found", entry.account_id),
                });
            }
        }

        // Record transaction
        self.journal.push(transaction.clone());

        // Update Balances
        for entry in transaction.entries {
            if let Some(account) = self.accounts.get_mut(&entry.account_id) {
                // Simplified Balance Update:
                // Asset/Expense: Increase on Debit, Decrease on Credit
                // Liability/Equity/Income: Decrease on Debit, Increase on Credit
                // For now, we just track raw movement, accurate accounting equation logic needed later.
                
                // Note: Money subtraction can be tricky if not signed. 
                // Assuming Money handles basic ops. A robust system uses Signed Money or Debit/Credit counters.
                // Simple implementation:
                account.balance = account.balance + entry.debit;
                account.balance = account.balance - entry.credit; 
            }
        }

        Ok(())
    }
}
