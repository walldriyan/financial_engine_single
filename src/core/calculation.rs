use crate::core::money::Money;
use crate::core::errors::{EngineResult, EngineError};
use crate::types::cart::Cart;

/// ============================================================================
/// 🧮 Calculation Engine (ගණනය කිරීමේ යන්ත්‍රය)
/// ============================================================================
/// සියලුම බදු, වට්ටම් සහ ගාස්තු ගණනය කිරීමේ මධ්‍යස්ථානය.
/// මෙය pipeline එකක් ලෙස ක්‍රියා කරයි.

pub struct CalculationEngine {
    // Configuration fields usually go here (e.g. RoundingMode)
}

impl CalculationEngine {
    pub fn new() -> Self {
        CalculationEngine {}
    }

    /// 🚀 ගණනය කරන්න (Calculate)
    /// මෙය සම්පූර්ණ ක්‍රියාවලිය පාලනය කරයි.
    pub fn calculate(&self, cart: &Cart, rules: &[Box<dyn crate::rules::traits::Rule + Send + Sync>]) -> EngineResult<CalculationResult> {
        // 1. Subtotal ලබා ගැනීම
        let subtotal = cart.subtotal();

        // 2. රීති ක්‍රියාත්මක කිරීම (Rules Execution)
        let mut discount_total = Money::zero();
        let mut tax_total = Money::zero();
        let mut fees_total = Money::zero();

        // Sort rules by priority (High to Low)
        // Note: In a real engine, we might want to clone the rules or sort indices to avoid mutating the input ref locally if needed,
        // but here we iterate. To strictly follow priority, we should collect and sort.
        // For now, let's assume the caller passes them sorted or we iterate simply.
        // A better approach: 
        let mut sorted_rules: Vec<&Box<dyn crate::rules::traits::Rule + Send + Sync>> = rules.iter().collect();
        sorted_rules.sort_by(|a, b| b.priority().cmp(&a.priority()));

        for rule in sorted_rules {
            if rule.can_apply(cart) {
                let actions = rule.apply(cart)?;
                for action in actions {
                    match action {
                        crate::rules::traits::RuleAction::Discount(amount) => {
                            discount_total = discount_total + amount;
                        },
                        crate::rules::traits::RuleAction::Tax(amount) => {
                            tax_total = tax_total + amount;
                        },
                        crate::rules::traits::RuleAction::Fee(amount) => {
                            fees_total = fees_total + amount;
                        },
                        _ => {} // Handle others later
                    }
                }
            }
        }

        // 3. අවසාන එකතුව (Total Calculation)
        // Total = Subtotal - Discounts + Taxes + Fees
        let total = subtotal - discount_total + tax_total + fees_total;

        // Example error check
        if total.is_negative() {
             return Err(EngineError::Calculation {
                 code: "NEGATIVE_TOTAL".to_string(),
                 message: "Total cannot be negative".to_string(),
             });
        }

        Ok(CalculationResult {
            subtotal,
            discount_total,
            tax_total,
            grand_total: total,
        })
    }
}

use serde::{Deserialize, Serialize};

/// 📊 ප්‍රතිඵලය (Result)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub subtotal: Money,
    pub discount_total: Money,
    pub tax_total: Money,
    pub grand_total: Money,
}
