use crate::rules::traits::{Rule, RuleAction};
use crate::types::cart::Cart;
use crate::core::errors::EngineResult;
use crate::rules::conditions::Condition;

/// ============================================================================
/// 📉 Percentage Discount (ප්‍රතිශත වට්ටම්)
/// ============================================================================

pub struct PercentageDiscount {
    name: String,
    percentage: f64,
    condition: Condition,
    priority: i32,
}

impl PercentageDiscount {
    pub fn new(name: &str, percentage: f64, condition: Condition) -> Self {
        PercentageDiscount {
            name: name.to_string(),
            percentage,
            condition,
            priority: 10, // Default priority
        }
    }
}

impl Rule for PercentageDiscount {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_apply(&self, cart: &Cart) -> bool {
        self.condition.evaluate(cart)
    }

    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let subtotal = cart.subtotal();
        // Calculate discount amount: subtotal * (percentage / 100)
        // We can use Money::sub_percentage logic but here we need the AMOUNT to subtract
        let original = subtotal;
        let discounted = subtotal.sub_percentage(self.percentage);
        let discount_amount = original - discounted;

        Ok(vec![RuleAction::Discount(discount_amount)])
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
