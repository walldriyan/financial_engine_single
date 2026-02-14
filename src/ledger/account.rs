use crate::core::money::Money;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// 📒 Ledger Account (ගිණුම)
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    Asset,      // වත්කම් (Bank, Cash, Inventory)
    Liability,  // වගකීම් (Loans, Payable)
    Equity,     // හිමිකම් (Capital, Retained Earnings)
    Income,     // ආදායම් (Sales, Service Revenue)
    Expense,    // වියදම් (Salaries, Rent)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: AccountType,
    pub currency_code: String,
    pub balance: Money,
}

impl Account {
    pub fn new(id: &str, name: &str, account_type: AccountType) -> Self {
        Account {
            id: id.to_string(),
            name: name.to_string(),
            account_type,
            currency_code: "LKR".to_string(),
            balance: Money::zero(),
        }
    }
}
