use crate::core::money::Money;
use std::ops::{Div, Mul};

/// ============================================================================
/// 🏛️ VAT (Value Added Tax)
/// ============================================================================

pub struct Vat {
    rate: f64,
}

impl Vat {
    pub fn new(rate: f64) -> Self {
        Vat { rate }
    }

    pub fn calculate(&self, amount: Money) -> Money {
        amount.mul(self.rate as i64).div(100)
    }
}
