use serde::{Deserialize, Serialize};
use crate::types::item::Item;
use crate::types::currency::Currency;
use crate::core::money::Money;

/// ============================================================================
/// 🛒 Cart (කරත්තය) - ගනුදෙනු එකතුව
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    /// අද්විතීය අංකය (Transaction ID)
    pub id: String,

    /// පාරිභෝගිකයා (Customer ID - Optional)
    pub customer_id: Option<String>,

    /// අයිතම ලැයිස්තුව (List of Items)
    pub items: Vec<Item>,

    /// මූලික මුදල් වර්ගය (Base Currency)
    pub currency: Currency,
}

impl Cart {
    /// 🆕 අලුත් කරත්තයක් (New Cart)
    pub fn new() -> Self {
        Cart {
            id: uuid::Uuid::new_v4().to_string(),
            customer_id: None,
            items: Vec::new(),
            currency: Currency::LKR,
        }
    }

    /// ➕ අයිතමයක් එකතු කරන්න (Add Item)
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    /// 💰 උප එකතුව (Subtotal without tax/discounts)
    pub fn subtotal(&self) -> Money {
        let mut total = Money::zero();
        for item in &self.items {
            // Note: Currency conversion would happen here if mixed currencies
            if item.currency == self.currency {
                total = total + item.total();
            }
        }
        total
    }
}
