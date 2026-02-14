use crate::audit::logger::{LogLevel, Logger}; // Centralized Audit
use crate::core::errors::{EngineError, EngineResult};
use crate::core::money::Money;
use crate::refund::types::{RefundRequest, RefundResult, RefundType};
use crate::types::cart::Cart;
use std::ops::Mul;

/// ============================================================================
/// 🔄 Refund Processor (ආපසු ගෙවීම් යන්ත්‍රය)
/// ============================================================================
/// Refund logic පාලනය කරයි.
/// State history සහ Audit සමඟ සම්බන්ධ වේ.

pub struct RefundProcessor {
    logger: Logger,
}

impl RefundProcessor {
    pub fn new() -> Self {
        RefundProcessor {
            logger: Logger::new(),
        }
    }

    /// 🚀 Process Refund (ආපසු ගෙවීම ක්‍රියාත්මක කරන්න)
    pub fn process(
        &self,
        original_cart: &Cart,
        request: &RefundRequest,
    ) -> EngineResult<RefundResult> {
        // 1. Validate Transaction ID
        if original_cart.id != request.original_transaction_id {
            return Err(EngineError::Validation {
                message: "Transaction ID mismatch (Field: transaction_id)".to_string(),
            });
        }

        // 2. Audit Log Start
        self.logger.log(
            LogLevel::Info,
            "REFUND",
            "START",
            &format!("Processing refund for {}", original_cart.id),
        )?;

        // 3. Logic for Full vs Partial
        // For simplicity, let's calculate based on items
        let mut refund_total = Money::zero();
        let refund_type = RefundType::Partial;

        // Check if full refund
        // TODO: Comparison logic implementation

        for (item_id, qty) in &request.items_to_refund {
            if let Some(item) = original_cart.items.iter().find(|i| i.id == *item_id) {
                if *qty > item.quantity {
                    return Err(EngineError::Validation {
                        message: "Refund quantity exceeds original (Field: quantity)".to_string(),
                    });
                }

                // Calculate refund amount for this item
                // Simple calculation: Unit Price * Qty
                // In real world: Need to re-apply rules inversely!
                let item_refund = item.price.mul(*qty as i64);
                refund_total = refund_total + item_refund;
            } else {
                return Err(EngineError::NotFound {
                    resource: "Item".to_string(),
                    id: item_id.clone(),
                });
            }
        }

        // 4. Audit Log Success
        self.logger.log(
            LogLevel::Audit,
            "REFUND",
            "SUCCESS",
            &format!("Refunded {}", refund_total),
        )?;

        Ok(RefundResult {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_id: original_cart.id.clone(),
            timestamp: chrono::Utc::now(),
            refund_amount: refund_total,
            refund_type,
            new_cart_state: None, // TODO: Return updated cart for partial refunds
        })
    }
}
