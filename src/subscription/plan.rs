use crate::core::money::Money;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// 📅 Subscription Plan (දායකත්ව සැලැස්ම)
/// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Yearly,
    /// විශේෂ කාල පරාසයක් (දින ගණන)
    Custom {
        days: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub price: Money,
    pub cycle: BillingCycle,
}

impl Plan {
    pub fn new(name: &str, price: Money, cycle: BillingCycle) -> Self {
        Plan {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            price,
            cycle,
        }
    }
}
