use serde::{Deserialize, Serialize};
use crate::types::cart::Cart;
use crate::core::money::Money;
use chrono::{DateTime, Utc};

/// ============================================================================
/// 🔄 Refund Types (ආපසු ගෙවීම් වර්ග)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefundType {
    /// සම්පූර්ණ මුදල ආපසු ගෙවීම
    Full,
    /// කොටසක් පමණක් ආපසු ගෙවීම
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub original_transaction_id: String,
    pub items_to_refund: Vec<(String, f64)>, // Item ID, Quantity
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResult {
    pub id: String,
    pub transaction_id: String,
    pub timestamp: DateTime<Utc>,
    pub refund_amount: Money,
    pub refund_type: RefundType,
    pub new_cart_state: Option<Cart>, // State after partial refund
}
