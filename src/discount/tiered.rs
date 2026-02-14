use crate::rules::traits::{Rule, RuleAction};
use crate::types::cart::Cart;
use crate::core::errors::EngineResult;

/// ============================================================================
/// 📶 Tiered Discount (ශ්‍රේණිගත වට්ටම්)
/// ============================================================================
/// ප්‍රමාණය අනුව වෙනස් වන වට්ටම්.
/// Ex: Buy 5 -> 5% Off, Buy 10 -> 10% Off

pub struct Tier {
    pub min_qty: f64,
    pub percentage: f64,
}

pub struct TieredDiscount {
    name: String,
    tiers: Vec<Tier>,
    priority: i32,
}

impl TieredDiscount {
    pub fn new(name: &str, tiers: Vec<Tier>) -> Self {
        TieredDiscount {
            name: name.to_string(),
            tiers, // Should be sorted by min_qty desc
            priority: 5,
        }
    }
}

impl Rule for TieredDiscount {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_apply(&self, _cart: &Cart) -> bool {
        true // Always runs, but checks tiers inside
    }

    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let total_qty: f64 = cart.items.iter().map(|i| i.quantity).sum();
        
        // Find the matching tier (highest matching min_qty)
        // Assuming tiers are sorted descending
        for tier in &self.tiers {
            if total_qty >= tier.min_qty {
                let subtotal = cart.subtotal();
                let original = subtotal;
                let discounted = subtotal.sub_percentage(tier.percentage);
                let discount_amount = original - discounted;
                
                return Ok(vec![RuleAction::Discount(discount_amount)]);
            }
        }

        Ok(vec![])
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
