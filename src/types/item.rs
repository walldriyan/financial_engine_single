use serde::{Deserialize, Serialize};
use crate::core::money::Money;
use crate::types::currency::Currency;
use uuid::Uuid;
use std::ops::Mul;

/// ============================================================================
/// 📦 Item (අයිතමය) - භාණ්ඩ හෝ සේවා
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// අද්විතීය අංකය (Unique ID)
    pub id: String,

    /// නම (Name)
    pub name: String,

    /// ඒකක මිල (Unit Price)
    pub price: Money,

    /// ප්‍රමාණය (Quantity)
    pub quantity: f64,

    /// මුදල් වර්ගය (Currency)
    pub currency: Currency,

    /// අමතර දත්ත (Metadata)
    /// Ex: category, SKU, taxable status
    pub metadata: std::collections::HashMap<String, String>,
}

impl Item {
    /// ➕ අලුත් අයිතමයක් සාදන්න
    pub fn new(name: &str, price: Money, quantity: f64) -> Self {
        Item {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            price,
            quantity,
            currency: Currency::LKR, // Default to LKR
            metadata: std::collections::HashMap::new(),
        }
    }

    /// 💰 මුළු වටිනාකම (Total Value)
    /// Price * Quantity
    pub fn total(&self) -> Money {
        self.price.mul(self.quantity as i64)
    }
}
