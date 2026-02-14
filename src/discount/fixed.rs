use crate::rules::traits::{Rule, RuleAction};
use crate::types::cart::Cart;
use crate::core::errors::EngineResult;
use crate::core::money::Money;
use crate::rules::conditions::Condition;

/// ============================================================================
/// 💵 Fixed Discount (ස්ථාවර වට්ටම්)
/// ============================================================================
/// නිශ්චිත මුදලක් අඩු කිරීම (Ex: Rs. 100 Off)

pub struct FixedDiscount {
    name: String,
    amount: Money,
    condition: Condition,
    priority: i32,
}

impl FixedDiscount {
    pub fn new(name: &str, amount: Money, condition: Condition) -> Self {
        FixedDiscount {
            name: name.to_string(),
            amount,
            condition,
            priority: 10,
        }
    }
}

impl Rule for FixedDiscount {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_apply(&self, cart: &Cart) -> bool {
        self.condition.evaluate(cart)
    }

    fn apply(&self, _cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        // Fixed amount discount
        Ok(vec![RuleAction::Discount(self.amount)])
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
