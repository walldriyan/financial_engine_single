use crate::audit::logger::{LogLevel, Logger};
use crate::core::errors::{EngineError, EngineResult};
use crate::core::money::Money;
use crate::refund::types::{RefundRequest, RefundResult, RefundType};
use crate::rules::mixed_scenarios::CartCalculation;
use crate::types::cart::Cart;

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

    /// 🚀 Process Refund ( නිවැරදි ක්‍රමය )
    /// Original Cart එකෙන් Quantity ප්‍රමාණය සහ Original Calculation එකෙන් මුදල ගණනය කරයි.
    /// Discount සහ Tax ස්වයංක්‍රීයව අදාළ වේ.
    pub fn process(
        &self,
        original_cart: &Cart,
        original_calculation: &CartCalculation,
        request: &RefundRequest,
    ) -> EngineResult<RefundResult> {
        let mut total_refund = Money::zero();

        // Audit Log Start
        self.logger.log(
            LogLevel::Info,
            "REFUND",
            "START",
            &format!("Processing refund for {}", original_cart.id),
        )?;

        for (item_id, return_qty) in &request.items_to_refund {
            // 1. Find Item in Cart (to verify Qty)
            let original_item = original_cart
                .items
                .iter()
                .find(|i| i.id == *item_id || i.name == *item_id)
                .ok_or_else(|| EngineError::NotFound {
                    resource: "Item".to_string(),
                    id: item_id.clone(),
                })?;

            if *return_qty > original_item.quantity {
                return Err(EngineError::Validation {
                    message: format!(
                        "Refund qty {} exceeds original {}",
                        return_qty, original_item.quantity
                    ),
                });
            }

            // 2. Find Calculation Result (to get Paid Amount)
            let calc_result = original_calculation
                .items
                .iter()
                .find(|i| i.item_id == *item_id || i.item_id == original_item.id)
                .ok_or_else(|| EngineError::Validation {
                    message: format!("No calculation found for item {}", item_id),
                })?;

            // 3. Pro-rata Logic (Proportional Refund)
            // Refund = Total Paid For Line * (Return Qty / Original Qty)
            let ratio = return_qty / original_item.quantity;
            let refund_amount = calc_result.total.mul_ratio(ratio);

            total_refund = total_refund + refund_amount;
        }

        // Audit Log Success
        self.logger.log(
            LogLevel::Info,
            "REFUND",
            "SUCCESS",
            &format!("Refunded {}", total_refund),
        )?;

        Ok(RefundResult {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_id: original_cart.id.clone(),
            timestamp: chrono::Utc::now(),
            refund_amount: total_refund,
            refund_type: RefundType::Partial,
            new_cart_state: None,
        })
    }
}
