use serde::{Deserialize, Serialize};
use crate::types::cart::Cart;
use crate::core::calculation::CalculationResult;
use chrono::{DateTime, Utc};

/// ============================================================================
/// 💾 Snapshot (ක්ෂණික ඡායාරූපය) - Immutable State
/// ============================================================================
/// පද්ධතියේ කිසිම දෙයක් වෙනස් වූ විට, අපි අලුත් snapshot එකක් සාදන්නෙමු.
/// මෙය history tracking සහ rollback සඳහා වැදගත් වේ.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub cart: Cart,
    pub calculation: Option<CalculationResult>,
    pub version: u64,
}

impl StateSnapshot {
    /// 📸 අලුත් snapshot එකක් ගන්න
    pub fn new(cart: Cart, calculation: Option<CalculationResult>, version: u64) -> Self {
        StateSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            cart,
            calculation,
            version,
        }
    }
}
