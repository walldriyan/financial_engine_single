use crate::core::money::Money;
use crate::types::cart::Cart;
use crate::types::item::Item;
use crate::types::currency::Currency;
use crate::core::calculation::{CalculationEngine, CalculationResult};
use crate::core::errors::EngineResult;
use crate::core::rounding::RoundingMode;

/// ============================================================================
/// 🌐 API Facade (බාහිර මුහුණත)
/// ============================================================================
/// පද්ධතිය භාවිතා කරන අයට පහසුවෙන් වැඩ කිරීමට මෙය උදව් වේ.
/// සංකීර්ණ දේවල් සඟවා සරල අතුරු මුහුණතක් ලබා දෙයි.

use crate::rules::traits::Rule;
use crate::ledger::journal::GeneralLedger;
use crate::inventory::stock::InventoryManager;

pub struct FinancialEngine {
    pub cart: Cart,
    pub calculator: CalculationEngine,
    pub rounding: RoundingMode,
    pub rules: Vec<Box<dyn Rule + Send + Sync>>,
    
    // 🌍 Advanced Modules
    pub ledger: GeneralLedger,
    pub inventory: InventoryManager,
}

impl FinancialEngine {
    /// 🚀 අලුත් එන්ජිමක් පණගන්වන්න (Initialize)
    pub fn new() -> Self {
        FinancialEngine {
            cart: Cart::new(),
            calculator: CalculationEngine::new(),
            rounding: RoundingMode::Standard,
            rules: Vec::new(),
            ledger: GeneralLedger::new(),
            inventory: InventoryManager::new(),
        }
    }

    /// ➕ භාණ්ඩයක් එකතු කරන්න (Add Item)
    pub fn add_item(&mut self, name: &str, price: f64, quantity: f64) -> &mut Self {
        let money_price = Money::from_float(price);
        let item = Item::new(name, money_price, quantity);
        self.cart.add_item(item);
        self
    }

    /// ➕ රීතියක් එකතු කරන්න (Add Rule)
    pub fn add_rule(&mut self, rule: Box<dyn Rule + Send + Sync>) -> &mut Self {
        self.rules.push(rule);
        self
    }

    /// 💱 මුදල් ඒකකය මාරු කරන්න (Set Currency)
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.cart.currency = currency;
        self
    }

    /// 🔢 වට කරන ක්‍රමය වෙනස් කරන්න (Set Rounding)
    pub fn set_rounding(&mut self, mode: RoundingMode) -> &mut Self {
        self.rounding = mode;
        self
    }

    /// 💰 ගණනය කරන්න (Calculate Total)
    pub fn calculate(&self) -> EngineResult<CalculationResult> {
        self.calculator.calculate(&self.cart, &self.rules)
    }

    /// 🏦 Ledger Access
    pub fn ledger(&mut self) -> &mut GeneralLedger {
        &mut self.ledger
    }

    /// 📦 Inventory Access
    pub fn inventory(&mut self) -> &mut InventoryManager {
        &mut self.inventory
    }
}
