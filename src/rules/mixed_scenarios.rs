use crate::core::errors::EngineResult;
use crate::core::money::Money;
use crate::types::cart::Cart;
// Rule, RuleAction removed as unused
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul};

/// ============================================================================
/// 🎯 Advanced Mixed Discount/Tax Engine (උසස් මිශ්‍ර වට්ටම්/බදු එන්ජිම)
/// ============================================================================
/// Supports:
/// - Mix tax rates per product/category
/// - Mix discount types (%, fixed, tiered, BOGO)
/// - Product-wise individual rules
/// - Stacking/non-stacking discounts
/// - Tax-on-discount / Discount-on-tax scenarios

/// 🏷️ Product-Level Tax Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTaxConfig {
    pub product_id: String,
    pub tax_rates: Vec<TaxRate>,
    pub tax_exempt: bool,
    pub tax_included_in_price: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub name: String,
    pub rate: f64,            // Percentage
    pub jurisdiction: String, // Country/State
    pub applies_to: TaxAppliesTo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaxAppliesTo {
    All,
    Category(String),
    Product(String),
    Region(String),
}

/// 🎁 Product-Level Discount Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDiscountConfig {
    pub product_id: String,
    pub discounts: Vec<DiscountRule>,
    pub stackable: bool,
    pub max_discount_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountRule {
    pub id: String,
    pub name: String,
    pub discount_type: DiscountType,
    pub priority: i32,
    pub conditions: Vec<DiscountCondition>,
    pub stackable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountType {
    FixedAmount(i64), // Cents
    Percentage(f64),  // %
    BuyXGetY {
        buy: f64,
        get: f64,
        free_percent: f64,
    },
    Tiered(Vec<TierLevel>),
    Bundle {
        items: Vec<String>,
        discount_percent: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierLevel {
    pub min_qty: f64,
    pub max_qty: Option<f64>,
    pub discount_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountCondition {
    MinQuantity(f64),
    MinAmount(i64),
    CustomerGroup(String),
    DateRange { from: String, to: String },
    FirstPurchase,
    PromoCode(String),
    CartContains(String),
}

/// 🧮 Mixed Scenario Calculator (මිශ්‍ර ගණනය කරන්නා)
pub struct MixedScenarioEngine {
    product_taxes: std::collections::HashMap<String, ProductTaxConfig>,
    product_discounts: std::collections::HashMap<String, ProductDiscountConfig>,
    global_tax_rates: Vec<TaxRate>,
    calculation_order: CalculationOrder,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalculationOrder {
    /// Discount first, then tax on discounted amount
    DiscountFirst,
    /// Tax first, then discount on taxed amount  
    TaxFirst,
    /// Tax on original, discount on original (independent)
    Parallel,
}

impl MixedScenarioEngine {
    pub fn new() -> Self {
        MixedScenarioEngine {
            product_taxes: std::collections::HashMap::new(),
            product_discounts: std::collections::HashMap::new(),
            global_tax_rates: Vec::new(),
            calculation_order: CalculationOrder::DiscountFirst,
        }
    }

    /// Set calculation order
    pub fn set_calculation_order(&mut self, order: CalculationOrder) {
        self.calculation_order = order;
    }

    /// Add global tax rate
    pub fn add_global_tax(&mut self, tax: TaxRate) {
        self.global_tax_rates.push(tax);
    }

    /// Add product-specific tax config
    pub fn add_product_tax(&mut self, config: ProductTaxConfig) {
        self.product_taxes.insert(config.product_id.clone(), config);
    }

    /// Add product-specific discount config
    pub fn add_product_discount(&mut self, config: ProductDiscountConfig) {
        self.product_discounts
            .insert(config.product_id.clone(), config);
    }

    /// 💰 Calculate for a single item
    pub fn calculate_item(
        &self,
        item_id: &str,
        unit_price: Money,
        quantity: f64,
        promo_codes: &[String],
    ) -> EngineResult<ItemCalculation> {
        let base_amount = unit_price * (quantity as i64);

        // Get applicable discounts
        let discount_amount =
            self.calculate_item_discount(item_id, &base_amount, quantity, promo_codes)?;

        // Calculate taxable amount based on order
        let taxable_amount = match self.calculation_order {
            CalculationOrder::DiscountFirst => base_amount - discount_amount,
            CalculationOrder::TaxFirst | CalculationOrder::Parallel => base_amount,
        };

        // Get applicable taxes
        let tax_amount = self.calculate_item_tax(item_id, &taxable_amount)?;

        // Final total
        let total = match self.calculation_order {
            CalculationOrder::DiscountFirst => taxable_amount + tax_amount,
            CalculationOrder::TaxFirst => base_amount + tax_amount - discount_amount,
            CalculationOrder::Parallel => base_amount - discount_amount + tax_amount,
        };

        Ok(ItemCalculation {
            item_id: item_id.to_string(),
            base_amount,
            discount_amount,
            tax_amount,
            total,
            discount_details: Vec::new(),
            tax_details: Vec::new(),
        })
    }

    /// Calculate discount for item
    fn calculate_item_discount(
        &self,
        item_id: &str,
        base_amount: &Money,
        quantity: f64,
        promo_codes: &[String],
    ) -> EngineResult<Money> {
        let mut total_discount = Money::zero();

        if let Some(config) = self.product_discounts.get(item_id) {
            let mut applied_non_stackable = false;

            // Sort by priority (higher first)
            let mut rules = config.discounts.clone();
            rules.sort_by(|a, b| b.priority.cmp(&a.priority));

            for rule in rules {
                // Check if we can still apply
                if applied_non_stackable && !rule.stackable {
                    continue;
                }

                // Check conditions
                let conditions_met =
                    self.check_conditions(&rule.conditions, quantity, base_amount, promo_codes);
                if !conditions_met {
                    continue;
                }

                // Calculate discount
                let discount = match &rule.discount_type {
                    DiscountType::FixedAmount(cents) => Money::from_cents(*cents),
                    DiscountType::Percentage(pct) => {
                        base_amount.sub_percentage(*pct) - *base_amount
                    }
                    DiscountType::BuyXGetY {
                        buy,
                        get,
                        free_percent,
                    } => {
                        let sets = (quantity / (*buy + *get)).floor();
                        let free_items = sets * get;
                        let unit_price = base_amount.div(quantity as i64);
                        let discount_per_free = unit_price
                            .mul((*free_percent / 100.0 * 100.0) as i64)
                            .div(100);
                        discount_per_free * (free_items as i64)
                    }
                    DiscountType::Tiered(tiers) => {
                        let mut tier_discount = Money::zero();
                        for tier in tiers {
                            let max = tier.max_qty.unwrap_or(f64::MAX);
                            if quantity >= tier.min_qty && quantity <= max {
                                tier_discount = base_amount.sub_percentage(tier.discount_percent);
                                tier_discount = *base_amount - tier_discount;
                                break;
                            }
                        }
                        tier_discount
                    }
                    DiscountType::Bundle { .. } => Money::zero(), // Bundle handled at cart level
                };

                total_discount = total_discount + discount.abs();

                if !rule.stackable {
                    applied_non_stackable = true;
                }
            }

            // Apply max discount cap
            if let Some(max_pct) = config.max_discount_percent {
                let max_discount = base_amount.mul((max_pct * 100.0) as i64).div(10000);
                if total_discount > max_discount {
                    total_discount = max_discount;
                }
            }
        }

        Ok(total_discount)
    }

    /// Calculate tax for item
    fn calculate_item_tax(&self, item_id: &str, taxable_amount: &Money) -> EngineResult<Money> {
        let mut total_tax = Money::zero();

        // Check product-specific taxes
        if let Some(config) = self.product_taxes.get(item_id) {
            if config.tax_exempt {
                return Ok(Money::zero());
            }

            for tax_rate in &config.tax_rates {
                let tax = taxable_amount
                    .mul((tax_rate.rate * 100.0) as i64)
                    .div(10000);
                total_tax = total_tax + tax;
            }
        } else {
            // Apply global taxes
            for tax_rate in &self.global_tax_rates {
                match &tax_rate.applies_to {
                    TaxAppliesTo::All => {
                        let tax = taxable_amount
                            .mul((tax_rate.rate * 100.0) as i64)
                            .div(10000);
                        total_tax = total_tax + tax;
                    }
                    TaxAppliesTo::Product(pid) if pid == item_id => {
                        let tax = taxable_amount
                            .mul((tax_rate.rate * 100.0) as i64)
                            .div(10000);
                        total_tax = total_tax + tax;
                    }
                    _ => {}
                }
            }
        }

        Ok(total_tax)
    }

    /// Check discount conditions
    fn check_conditions(
        &self,
        conditions: &[DiscountCondition],
        quantity: f64,
        amount: &Money,
        promo_codes: &[String],
    ) -> bool {
        if conditions.is_empty() {
            return true;
        }

        for condition in conditions {
            let met = match condition {
                DiscountCondition::MinQuantity(min) => quantity >= *min,
                DiscountCondition::MinAmount(cents) => amount.amount >= *cents,
                DiscountCondition::PromoCode(code) => promo_codes.contains(code),
                // Other conditions need external data
                _ => true,
            };
            if !met {
                return false;
            }
        }
        true
    }

    /// 📊 Calculate full cart
    pub fn calculate_cart(
        &self,
        cart: &Cart,
        promo_codes: &[String],
    ) -> EngineResult<CartCalculation> {
        let mut item_results = Vec::new();
        let mut subtotal = Money::zero();
        let mut total_discount = Money::zero();
        let mut total_tax = Money::zero();

        for item in &cart.items {
            let result = self.calculate_item(&item.id, item.price, item.quantity, promo_codes)?;

            subtotal = subtotal + result.base_amount;
            total_discount = total_discount + result.discount_amount;
            total_tax = total_tax + result.tax_amount;
            item_results.push(result);
        }

        let grand_total = subtotal - total_discount + total_tax;

        Ok(CartCalculation {
            items: item_results,
            subtotal,
            total_discount,
            total_tax,
            grand_total,
        })
    }
}

/// 📋 Item Calculation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCalculation {
    pub item_id: String,
    pub base_amount: Money,
    pub discount_amount: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub discount_details: Vec<DiscountDetail>,
    pub tax_details: Vec<TaxDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountDetail {
    pub rule_id: String,
    pub name: String,
    pub amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxDetail {
    pub name: String,
    pub rate: f64,
    pub amount: Money,
}

/// 📊 Cart Calculation Result  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartCalculation {
    pub items: Vec<ItemCalculation>,
    pub subtotal: Money,
    pub total_discount: Money,
    pub total_tax: Money,
    pub grand_total: Money,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_tax_discount() {
        let mut engine = MixedScenarioEngine::new();

        // Add 10% VAT globally
        engine.add_global_tax(TaxRate {
            name: "VAT".to_string(),
            rate: 10.0,
            jurisdiction: "LK".to_string(),
            applies_to: TaxAppliesTo::All,
        });

        // Add product discount
        engine.add_product_discount(ProductDiscountConfig {
            product_id: "PROD001".to_string(),
            discounts: vec![DiscountRule {
                id: "DISC001".to_string(),
                name: "10% Off".to_string(),
                discount_type: DiscountType::Percentage(10.0),
                priority: 1,
                conditions: vec![],
                stackable: true,
            }],
            stackable: true,
            max_discount_percent: Some(50.0),
        });

        let result = engine
            .calculate_item("PROD001", Money::new(100, 0), 1.0, &[])
            .unwrap();

        // Base: 100, Discount: 10, Taxable: 90, Tax: 9, Total: 99
        assert_eq!(result.base_amount.amount, 10000);
        assert_eq!(result.discount_amount.amount, 1000);
        assert_eq!(result.total.amount, 9900);
    }

    #[test]
    fn test_tiered_discount() {
        let mut engine = MixedScenarioEngine::new();

        engine.add_product_discount(ProductDiscountConfig {
            product_id: "PROD002".to_string(),
            discounts: vec![DiscountRule {
                id: "TIER001".to_string(),
                name: "Bulk Discount".to_string(),
                discount_type: DiscountType::Tiered(vec![
                    TierLevel {
                        min_qty: 10.0,
                        max_qty: Some(49.0),
                        discount_percent: 5.0,
                    },
                    TierLevel {
                        min_qty: 50.0,
                        max_qty: None,
                        discount_percent: 15.0,
                    },
                ]),
                priority: 1,
                conditions: vec![],
                stackable: false,
            }],
            stackable: false,
            max_discount_percent: None,
        });

        let result = engine
            .calculate_item("PROD002", Money::new(10, 0), 50.0, &[])
            .unwrap();

        // 50 items * Rs.10 = Rs.500, 15% off = Rs.75 discount
        assert_eq!(result.discount_amount.amount, 7500);
    }
}
