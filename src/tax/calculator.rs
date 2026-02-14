use crate::types::cart::Cart;
use crate::core::money::Money;
use crate::core::errors::EngineResult;
use std::ops::{Mul, Div};

/// ============================================================================
/// 🏛️ Tax Engine (බදු එන්ජිම)
/// ============================================================================
/// Multi-jurisdiction tax calculation support.

#[derive(Debug, Clone)]
pub struct TaxRule {
    pub name: String,
    pub percentage: f64,
    // TODO: Add jurisdiction logic
}

pub struct TaxCalculator {
    rules: Vec<TaxRule>,
}

impl TaxCalculator {
    pub fn new() -> Self {
        TaxCalculator {
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: TaxRule) {
        self.rules.push(rule);
    }

    /// 💰 බදු ගණනය කරන්න (Calculate Tax)
    pub fn calculate(&self, cart: &Cart) -> EngineResult<Money> {
        let mut total_tax = Money::zero();
        let taxable_amount = cart.subtotal(); // Assuming subtotal is taxable base

        for rule in &self.rules {
            // Simple VAT-style calculation
            // Tax = Amount * (Rate / 100)
            let tax_amount = taxable_amount.mul(rule.percentage as i64).div(100);
            total_tax = total_tax + tax_amount;
        }

        Ok(total_tax)
    }
}
