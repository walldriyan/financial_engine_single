use crate::rules::traits::{Rule, RuleAction};
use crate::types::cart::Cart;
use crate::core::errors::EngineResult;
use crate::core::money::Money;
use std::ops::Mul;

/// ============================================================================
/// 🏷️ Item Discount (භාණ්ඩයකට අදාළ වට්ටම්)
/// ============================================================================

pub struct ItemDiscount {
    name: String,
    target_item_name: String, // Or ID
    discount_amount: Money, // Fixed amount off per unit
    priority: i32,
}

impl ItemDiscount {
    pub fn new(name: &str, target_item_name: &str, amount: Money) -> Self {
        ItemDiscount {
            name: name.to_string(),
            target_item_name: target_item_name.to_string(),
            discount_amount: amount,
            priority: 20, // Higher priority than general
        }
    }
}

impl Rule for ItemDiscount {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_apply(&self, cart: &Cart) -> bool {
        // Check if cart contains the item
        cart.items.iter().any(|item| item.name == self.target_item_name)
    }

    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let mut actions = Vec::new();
        
        for item in &cart.items {
            if item.name == self.target_item_name {
                // Discount per unit * quantity
                // NOTE: Simply multiplying Money * f64 isn't standard in Money helper usually (usually i64).
                // Assuming Money handles it or we do logic manually.
                // For simplicity: (discount * quantity)
                
                let qty_int = item.quantity as i64; // Simple case
                let total_item_discount = self.discount_amount.mul(qty_int);
                actions.push(RuleAction::Discount(total_item_discount));
            }
        }

        Ok(actions)
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
