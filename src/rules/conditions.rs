use crate::core::money::Money;
use crate::types::cart::Cart;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// ⚡ Conditions (කොන්දේසි) - රීති සඳහා අවශ්‍ය කොන්දේසි
/// ============================================================================
/// රීතියක් ක්‍රියාත්මක විය යුත්තේ කවදාද යන්න මෙය තීරණය කරයි.
/// උදා: "භාණ්ඩ 5ට වඩා වැඩි නම්" හෝ "මුළු අගය 1000ට වැඩි නම්".

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    /// වඩා වැඩි (Greater Than) >
    Gt,
    /// වඩා අඩු (Less Than) <
    Lt,
    /// සමාන (Equal) ==
    Eq,
    /// වඩා වැඩි හෝ සමාන (Greater Than or Equal) >=
    Gte,
    /// වඩා අඩු හෝ සමාන (Less Than or Equal) <=
    Lte,
    /// ඇතුළත් (In) - List එකක තිබේ නම්
    In(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// කරත්තයේ මුළු වටිනාකම (Cart Subtotal)
    Subtotal {
        op: Operator,
        value: Money,
    },

    /// මුළු භාණ්ඩ ප්‍රමාණය (Total Item Quantity)
    TotalQuantity {
        op: Operator,
        value: f64,
    },

    /// විශේෂිත භාණ්ඩයක් තිබේද? (Contains Item?)
    HasItem {
        item_id: String,
        min_qty: f64,
    },

    /// සංකීර්ණ කොන්දේසි (Complex Logic)
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),

    /// සැමවිටම සත්‍ය වේ (Always True)
    Always,
}

impl Condition {
    /// 🕵️ කොන්දේසිය පරීක්ෂා කරන්න (Evaluate)
    pub fn evaluate(&self, cart: &Cart) -> bool {
        match self {
            Condition::Subtotal { op, value } => {
                let subtotal = cart.subtotal();
                match op {
                    Operator::Gt => subtotal > *value,
                    Operator::Lt => subtotal < *value,
                    Operator::Eq => subtotal == *value,
                    Operator::Gte => subtotal >= *value,
                    Operator::Lte => subtotal <= *value,
                    _ => false, // TODO: Implement other ops logic for Money
                }
            }
            Condition::TotalQuantity { op, value } => {
                let total_qty: f64 = cart.items.iter().map(|i| i.quantity).sum();
                match op {
                    Operator::Gt => total_qty > *value,
                    Operator::Lt => total_qty < *value,
                    Operator::Eq => (total_qty - *value).abs() < f64::EPSILON,
                    Operator::Gte => total_qty >= *value,
                    Operator::Lte => total_qty <= *value,
                    _ => false,
                }
            }
            Condition::And(conditions) => conditions.iter().all(|c| c.evaluate(cart)),
            Condition::Or(conditions) => conditions.iter().any(|c| c.evaluate(cart)),
            Condition::Not(condition) => !condition.evaluate(cart),
            _ => true, // Placeholder for other conditions
        }
    }
}
