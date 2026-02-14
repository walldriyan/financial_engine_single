use crate::rules::traits::{Rule, RuleAction};
use crate::types::cart::Cart;
use crate::core::errors::EngineResult;
use crate::core::money::Money;
use std::ops::Mul;

/// ============================================================================
/// 🎁 Promotions (විශේෂ දීමනා)
/// ============================================================================

/// 1. Buy N Items, Get M Items Free (or Discount equivalent)
pub struct BuyNGetFree {
    pub name: String,
    pub target_item: String,
    pub buy_qty: f64,
    pub free_qty: f64,
    pub priority: i32,
}

impl BuyNGetFree {
    pub fn new(name: &str, item: &str, buy: f64, get: f64) -> Self {
        BuyNGetFree {
            name: name.to_string(),
            target_item: item.to_string(),
            buy_qty: buy,
            free_qty: get,
            priority: 50, // High priority
        }
    }
}

impl Rule for BuyNGetFree {
    fn name(&self) -> &str { &self.name }
    fn priority(&self) -> i32 { self.priority }

    fn can_apply(&self, cart: &Cart) -> bool {
        cart.items.iter().any(|i| i.name == self.target_item && i.quantity >= self.buy_qty)
    }

    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let mut actions = Vec::new();
        for item in &cart.items {
            if item.name == self.target_item {
                // Logic: For every (Buy + Get) chunk, give Get free.
                // Ex: Buy 2 Get 1 Free. User puts 3 in cart. 
                // We discount 1 unit price.
                
                // If user put just 2, do they get 1 phantom item? 
                // Usually POS logic: You must scan 3 to get 1 free.
                // Let's assume input qty includes the free items.
                
                let set_size = self.buy_qty + self.free_qty;
                let num_sets = (item.quantity / set_size).floor();
                
                if num_sets > 0.0 {
                    let free_count = num_sets * self.free_qty;
                    let discount_amount = item.price.mul(free_count as i64);
                    actions.push(RuleAction::Discount(discount_amount));
                }
            }
        }
        Ok(actions)
    }
}

/// 2. Price Threshold Fixed Discount (If Price > X, Get Y Off)
pub struct PriceThresholdFixed {
    pub name: String,
    pub item_name: String,
    pub threshold: Money,
    pub discount: Money,
}

impl Rule for PriceThresholdFixed {
    fn name(&self) -> &str { &self.name }
    fn priority(&self) -> i32 { 40 }
    fn can_apply(&self, cart: &Cart) -> bool {
        cart.items.iter().any(|i| i.name == self.item_name && i.price > self.threshold)
    }
    
    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let mut actions = Vec::new();
        for item in &cart.items {
            if item.name == self.item_name && item.price > self.threshold {
                let total_disc = self.discount.mul(item.quantity as i64);
                actions.push(RuleAction::Discount(total_disc));
            }
        }
        Ok(actions)
    }
}

/// 3. Qty Threshold Percentage (If Qty > X, Get Y% Off)
pub struct QtyThresholdPercentage {
    pub name: String,
    pub item_name: String,
    pub threshold_qty: f64,
    pub percentage: f64,
}

impl Rule for QtyThresholdPercentage {
    fn name(&self) -> &str { &self.name }
    fn priority(&self) -> i32 { 30 }
    fn can_apply(&self, cart: &Cart) -> bool {
        cart.items.iter().any(|i| i.name == self.item_name && i.quantity > self.threshold_qty)
    }
    
    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let mut actions = Vec::new();
        for item in &cart.items {
            if item.name == self.item_name && item.quantity > self.threshold_qty {
                let item_total = item.total();
                let net_amount = item_total.sub_percentage(self.percentage); // Returns amount AFTER discount
                let disc_amt = item_total - net_amount;
                actions.push(RuleAction::Discount(disc_amt));
            }
        }
        Ok(actions)
    }
}

/// 4. Global Bill Qty Threshold Discount
pub struct GlobalQtyThreshold {
    pub name: String,
    pub threshold_qty: f64,
    pub discount_amount: Money,
}

impl Rule for GlobalQtyThreshold {
    fn name(&self) -> &str { &self.name }
    fn priority(&self) -> i32 { 10 }
    
    fn can_apply(&self, cart: &Cart) -> bool {
        let total_qty: f64 = cart.items.iter().map(|i| i.quantity).sum();
        total_qty > self.threshold_qty
    }
    
    fn apply(&self, _cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        Ok(vec![RuleAction::Discount(self.discount_amount.clone())])
    }
}
