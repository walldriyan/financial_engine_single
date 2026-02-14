use crate::core::errors::EngineResult;
use crate::core::money::Money;
use crate::rules::traits::{Rule, RuleAction};
use crate::types::cart::Cart;
use std::ops::{Div, Mul};

/// ============================================================================
/// 🏛️ Tax Rule (බදු රීති)
/// ============================================================================

pub enum TaxType {
    Percentage(f64),
    Fixed(Money),
}

pub struct TaxRule {
    name: String,
    tax_type: TaxType,
    priority: i32,
}

impl TaxRule {
    pub fn new_percentage(name: &str, rate: f64) -> Self {
        TaxRule {
            name: name.to_string(),
            tax_type: TaxType::Percentage(rate),
            priority: 5, // Lower priority, usually calculated last
        }
    }

    pub fn new_fixed(name: &str, amount: Money) -> Self {
        TaxRule {
            name: name.to_string(),
            tax_type: TaxType::Fixed(amount),
            priority: 5,
        }
    }
}

impl Rule for TaxRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_apply(&self, _cart: &Cart) -> bool {
        true // Applies generally, can be refined with conditions
    }

    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        match &self.tax_type {
            TaxType::Percentage(rate) => {
                let subtotal = cart.subtotal();
                // Subtotal * (rate / 100)
                let tax_amount = subtotal.mul(*rate as i64).div(100);
                Ok(vec![RuleAction::Tax(tax_amount)])
            }
            TaxType::Fixed(amount) => Ok(vec![RuleAction::Tax(amount.clone())]),
        }
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
