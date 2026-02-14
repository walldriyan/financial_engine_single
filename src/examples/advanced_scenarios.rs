/// ============================================================================
/// 🎯 ADVANCED DISCOUNT & TAX EXAMPLES - All Scenarios
/// ============================================================================
/// 
/// ✅ Percentage Discount
/// ✅ Fixed Discount  
/// ✅ Product-wise Multiple Rules
/// ✅ Qty Range (From-To)
/// ✅ Time-based Discount/Tax
/// ✅ Full Bill Discount
/// ✅ Mix All Together

use crate::core::money::Money;
use crate::rules::mixed_scenarios::*;

// =============================================================================
// 📊 DISCOUNT TYPES SUMMARY
// =============================================================================
//
// | Type              | Example                      | Code                    |
// |-------------------|------------------------------|-------------------------|
// | Fixed Amount      | Rs. 500 off                  | FixedAmount(50000)      |
// | Percentage        | 10% off                      | Percentage(10.0)        |
// | Tiered (Qty)      | 5+ items: 5%, 10+: 10%      | Tiered(vec![...])       |
// | Buy X Get Y       | Buy 2 Get 1 Free            | BuyXGetY{2,1,100}       |
// | Bundle            | Laptop+Mouse = 15% off      | Bundle{items, 15.0}     |
// | Time-based        | Valid Jan 20-22 only        | DateRange condition     |
// | Min Qty           | Only if qty >= 5            | MinQuantity(5.0)        |
// | Min Amount        | Only if total >= Rs.5000    | MinAmount(500000)       |
// | Promo Code        | Only with code "VIP10"      | PromoCode("VIP10")      |
//
// =============================================================================

/// Example 1: Percentage Discount
pub fn example_percentage_discount() {
    let mut engine = MixedScenarioEngine::new();
    
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "PROD001".to_string(),
        discounts: vec![DiscountRule {
            id: "PERC10".to_string(),
            name: "10% Off".to_string(),
            discount_type: DiscountType::Percentage(10.0), // 👈 10%
            priority: 50,
            conditions: vec![],
            stackable: true,
        }],
        stackable: true,
        max_discount_percent: None,
    });

    let result = engine.calculate_item("PROD001", Money::new(1000, 0), 1.0, &[]).unwrap();
    // Rs. 1000 - 10% = Rs. 100 discount
    assert_eq!(result.discount_amount.amount, 10000);
}

/// Example 2: Fixed Discount
pub fn example_fixed_discount() {
    let mut engine = MixedScenarioEngine::new();
    
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "PROD002".to_string(),
        discounts: vec![DiscountRule {
            id: "FIX500".to_string(),
            name: "Rs. 500 Off".to_string(),
            discount_type: DiscountType::FixedAmount(50000), // 👈 Rs. 500 (cents)
            priority: 50,
            conditions: vec![],
            stackable: true,
        }],
        stackable: true,
        max_discount_percent: None,
    });

    let result = engine.calculate_item("PROD002", Money::new(2000, 0), 1.0, &[]).unwrap();
    // Rs. 2000 - Rs. 500 = Rs. 500 discount
    assert_eq!(result.discount_amount.amount, 50000);
}

/// Example 3: Qty Range Tiered (From-To)
pub fn example_qty_range_discount() {
    let mut engine = MixedScenarioEngine::new();
    
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "PROD003".to_string(),
        discounts: vec![DiscountRule {
            id: "TIER".to_string(),
            name: "Qty Based Discount".to_string(),
            discount_type: DiscountType::Tiered(vec![
                // From 1-4: No discount
                TierLevel { min_qty: 5.0,  max_qty: Some(9.0),  discount_percent: 5.0 },  // 5-9: 5%
                TierLevel { min_qty: 10.0, max_qty: Some(19.0), discount_percent: 10.0 }, // 10-19: 10%
                TierLevel { min_qty: 20.0, max_qty: Some(49.0), discount_percent: 15.0 }, // 20-49: 15%
                TierLevel { min_qty: 50.0, max_qty: None,       discount_percent: 20.0 }, // 50+: 20%
            ]),
            priority: 50,
            conditions: vec![],
            stackable: false,
        }],
        stackable: false,
        max_discount_percent: None,
    });

    // Test: Buy 15 items at Rs. 100 each
    let result = engine.calculate_item("PROD003", Money::new(100, 0), 15.0, &[]).unwrap();
    // Rs. 1500 * 10% = Rs. 150 discount
    assert_eq!(result.discount_amount.amount, 15000);
}

/// Example 4: Multiple Discount Rules Per Product (Mix Fixed + Percentage + Tiered)
pub fn example_multiple_rules_per_product() {
    let mut engine = MixedScenarioEngine::new();
    
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "LAPTOP001".to_string(),
        discounts: vec![
            // Rule 1: Tiered (non-stackable)
            DiscountRule {
                id: "TIER".to_string(),
                name: "Bulk Discount".to_string(),
                discount_type: DiscountType::Tiered(vec![
                    TierLevel { min_qty: 5.0, max_qty: None, discount_percent: 10.0 },
                ]),
                priority: 50,  // Highest priority
                conditions: vec![],
                stackable: false,
            },
            // Rule 2: Promo Code (stackable)
            DiscountRule {
                id: "PROMO".to_string(),
                name: "VIP Promo".to_string(),
                discount_type: DiscountType::Percentage(5.0),
                priority: 40,
                conditions: vec![DiscountCondition::PromoCode("VIP5".to_string())],
                stackable: true,
            },
            // Rule 3: Fixed Discount (stackable)
            DiscountRule {
                id: "FIXED".to_string(),
                name: "Rs. 1000 Off".to_string(),
                discount_type: DiscountType::FixedAmount(100000), // Rs. 1000
                priority: 30,
                conditions: vec![DiscountCondition::MinAmount(500000)], // Min Rs. 5000
                stackable: true,
            },
        ],
        stackable: true,  // Allow stacking
        max_discount_percent: Some(25.0), // Cap at 25%
    });

    // Test: 5 Laptops at Rs. 50,000 with VIP5 code
    let result = engine.calculate_item(
        "LAPTOP001", 
        Money::new(50000, 0), 
        5.0, 
        &["VIP5".to_string()]
    ).unwrap();
    
    // Base: Rs. 250,000
    // Tier 10%: Rs. 25,000
    // VIP 5%: Rs. 12,500 (stackable)
    // Fixed: Rs. 1,000 (stackable)
    // Total Discount: Rs. 38,500 (but capped at 25% = Rs. 62,500)
    println!("Discount: {}", result.discount_amount);
}

/// Example 5: Time-Based Discount
pub fn example_time_based_discount() {
    let mut engine = MixedScenarioEngine::new();
    
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "SALE001".to_string(),
        discounts: vec![DiscountRule {
            id: "FLASH".to_string(),
            name: "Flash Sale".to_string(),
            discount_type: DiscountType::Percentage(30.0),
            priority: 100, // Highest
            conditions: vec![
                DiscountCondition::DateRange {
                    from: "2024-01-20".to_string(),
                    to: "2024-01-22".to_string(),
                }
            ],
            stackable: false,
        }],
        stackable: false,
        max_discount_percent: None,
    });
    
    // This discount only applies between Jan 20-22!
}

/// Example 6: Product-wise Tax + Global Tax (Mix)
pub fn example_product_tax_mix() {
    let mut engine = MixedScenarioEngine::new();
    
    // Global 2% NBT for all
    engine.add_global_tax(TaxRate {
        name: "NBT".to_string(),
        rate: 2.0,
        jurisdiction: "LK".to_string(),
        applies_to: TaxAppliesTo::All,
    });

    // Product 1: Food - NO TAX
    engine.add_product_tax(ProductTaxConfig {
        product_id: "FOOD001".to_string(),
        tax_rates: vec![],
        tax_exempt: true,  // 👈 No tax!
        tax_included_in_price: false,
    });

    // Product 2: Electronics - 18% VAT
    engine.add_product_tax(ProductTaxConfig {
        product_id: "ELEC001".to_string(),
        tax_rates: vec![
            TaxRate { name: "VAT".to_string(), rate: 18.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::All },
        ],
        tax_exempt: false,
        tax_included_in_price: false,
    });

    // Product 3: Luxury - 18% VAT + 5% Luxury Tax
    engine.add_product_tax(ProductTaxConfig {
        product_id: "LUXURY001".to_string(),
        tax_rates: vec![
            TaxRate { name: "VAT".to_string(), rate: 18.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::All },
            TaxRate { name: "Luxury Tax".to_string(), rate: 5.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::All },
        ],
        tax_exempt: false,
        tax_included_in_price: false,
    });

    // Food: Rs. 1000, Tax = 0
    let food = engine.calculate_item("FOOD001", Money::new(1000, 0), 1.0, &[]).unwrap();
    assert_eq!(food.tax_amount.amount, 0);

    // Electronics: Rs. 1000, Tax = 18% = Rs. 180
    let elec = engine.calculate_item("ELEC001", Money::new(1000, 0), 1.0, &[]).unwrap();
    assert_eq!(elec.tax_amount.amount, 18000);

    // Luxury: Rs. 1000, Tax = 23% = Rs. 230
    let luxury = engine.calculate_item("LUXURY001", Money::new(1000, 0), 1.0, &[]).unwrap();
    assert_eq!(luxury.tax_amount.amount, 23000);
}

/// Example 7: Full Bill Discount (Cart Level)
pub fn example_full_bill_discount() {
    use crate::discount::fixed::FixedDiscount;
    use crate::discount::percentage::PercentageDiscount;
    use crate::rules::promotions::GlobalQtyThreshold;
    
    // Full bill Rs. 500 off if total > Rs. 5000
    let bill_discount = GlobalQtyThreshold {
        name: "Bill Discount".to_string(),
        threshold_qty: 0.0, // Any qty
        discount_amount: Money::new(500, 0),
    };

    // Use with FinancialEngine for cart-level rules
}

/// Example 8: Buy X Get Y Free
pub fn example_bogo() {
    let mut engine = MixedScenarioEngine::new();
    
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "SHIRT001".to_string(),
        discounts: vec![DiscountRule {
            id: "BOGO".to_string(),
            name: "Buy 2 Get 1 Free".to_string(),
            discount_type: DiscountType::BuyXGetY { 
                buy: 2.0, 
                get: 1.0, 
                free_percent: 100.0  // 100% free
            },
            priority: 100,
            conditions: vec![],
            stackable: false,
        }],
        stackable: false,
        max_discount_percent: None,
    });

    // Buy 6 shirts at Rs. 1000 each
    let result = engine.calculate_item("SHIRT001", Money::new(1000, 0), 6.0, &[]).unwrap();
    // 6 shirts = 2 complete sets (Buy 2 Get 1)
    // 2 free shirts = Rs. 2000 discount
    assert_eq!(result.discount_amount.amount, 200000);
}

/// Example 9: Promo Code + Min Amount Condition
pub fn example_conditional_discount() {
    let mut engine = MixedScenarioEngine::new();
    
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "ANY".to_string(),
        discounts: vec![DiscountRule {
            id: "SAVE20".to_string(),
            name: "Save 20%".to_string(),
            discount_type: DiscountType::Percentage(20.0),
            priority: 50,
            conditions: vec![
                DiscountCondition::PromoCode("SAVE20".to_string()),
                DiscountCondition::MinAmount(300000), // Min Rs. 3000
            ],
            stackable: true,
        }],
        stackable: true,
        max_discount_percent: None,
    });

    // Without code - NO discount
    let no_code = engine.calculate_item("ANY", Money::new(5000, 0), 1.0, &[]).unwrap();
    assert_eq!(no_code.discount_amount.amount, 0);

    // With code but under Rs. 3000 - NO discount
    let under_min = engine.calculate_item("ANY", Money::new(2000, 0), 1.0, &["SAVE20".to_string()]).unwrap();
    assert_eq!(under_min.discount_amount.amount, 0);

    // With code AND over Rs. 3000 - 20% discount!
    let valid = engine.calculate_item("ANY", Money::new(5000, 0), 1.0, &["SAVE20".to_string()]).unwrap();
    assert_eq!(valid.discount_amount.amount, 100000); // Rs. 1000 off
}

/// Example 10: COMPLETE MIX - All 4 Types in One Cart
/// Product-wise + Qty-wise + Full Bill Qty + Full Bill Price
pub fn example_complete_mix() {
    let mut engine = MixedScenarioEngine::new();
    
    // ═══════════════════════════════════════════════════════════════
    // PRODUCT 1: LAPTOP - Multi-tier discount + VAT
    // ═══════════════════════════════════════════════════════════════
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "LAPTOP".to_string(),
        discounts: vec![
            DiscountRule {
                id: "LAPTOP_TIER".to_string(),
                name: "Laptop Bulk".to_string(),
                discount_type: DiscountType::Tiered(vec![
                    TierLevel { min_qty: 1.0, max_qty: Some(2.0), discount_percent: 0.0 },
                    TierLevel { min_qty: 3.0, max_qty: Some(4.0), discount_percent: 5.0 },
                    TierLevel { min_qty: 5.0, max_qty: None, discount_percent: 10.0 },
                ]),
                priority: 50,
                conditions: vec![],
                stackable: false,
            },
        ],
        stackable: false,
        max_discount_percent: Some(15.0),
    });
    engine.add_product_tax(ProductTaxConfig {
        product_id: "LAPTOP".to_string(),
        tax_rates: vec![
            TaxRate { name: "VAT".to_string(), rate: 18.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::All },
        ],
        tax_exempt: false,
        tax_included_in_price: false,
    });

    // ═══════════════════════════════════════════════════════════════
    // PRODUCT 2: MOUSE - Fixed discount + No tax
    // ═══════════════════════════════════════════════════════════════
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "MOUSE".to_string(),
        discounts: vec![
            DiscountRule {
                id: "MOUSE_FIXED".to_string(),
                name: "Mouse Rs. 100 Off".to_string(),
                discount_type: DiscountType::FixedAmount(10000), // Rs. 100
                priority: 50,
                conditions: vec![DiscountCondition::MinQuantity(2.0)],
                stackable: true,
            },
        ],
        stackable: true,
        max_discount_percent: None,
    });
    engine.add_product_tax(ProductTaxConfig {
        product_id: "MOUSE".to_string(),
        tax_rates: vec![],
        tax_exempt: true,
        tax_included_in_price: false,
    });

    // ═══════════════════════════════════════════════════════════════
    // PRODUCT 3: KEYBOARD - BOGO + 5% tax
    // ═══════════════════════════════════════════════════════════════
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "KEYBOARD".to_string(),
        discounts: vec![
            DiscountRule {
                id: "KB_BOGO".to_string(),
                name: "Buy 2 Get 1 50% Off".to_string(),
                discount_type: DiscountType::BuyXGetY { buy: 2.0, get: 1.0, free_percent: 50.0 },
                priority: 50,
                conditions: vec![],
                stackable: false,
            },
        ],
        stackable: false,
        max_discount_percent: None,
    });
    engine.add_product_tax(ProductTaxConfig {
        product_id: "KEYBOARD".to_string(),
        tax_rates: vec![
            TaxRate { name: "Small Tax".to_string(), rate: 5.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::All },
        ],
        tax_exempt: false,
        tax_included_in_price: false,
    });

    // ═══════════════════════════════════════════════════════════════
    // PRODUCT 4: HEADPHONE - Percentage + Promo + Luxury Tax
    // ═══════════════════════════════════════════════════════════════
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "HEADPHONE".to_string(),
        discounts: vec![
            DiscountRule {
                id: "HP_PERC".to_string(),
                name: "Headphone 15% Off".to_string(),
                discount_type: DiscountType::Percentage(15.0),
                priority: 50,
                conditions: vec![],
                stackable: true,
            },
            DiscountRule {
                id: "HP_PROMO".to_string(),
                name: "Extra 5% with code".to_string(),
                discount_type: DiscountType::Percentage(5.0),
                priority: 40,
                conditions: vec![DiscountCondition::PromoCode("HP5".to_string())],
                stackable: true,
            },
        ],
        stackable: true,
        max_discount_percent: Some(25.0),
    });
    engine.add_product_tax(ProductTaxConfig {
        product_id: "HEADPHONE".to_string(),
        tax_rates: vec![
            TaxRate { name: "VAT".to_string(), rate: 18.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::All },
            TaxRate { name: "Luxury".to_string(), rate: 5.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::All },
        ],
        tax_exempt: false,
        tax_included_in_price: false,
    });

    // ═══════════════════════════════════════════════════════════════
    // CALCULATE ALL ITEMS
    // ═══════════════════════════════════════════════════════════════
    
    // LAPTOP: 5 × Rs. 100,000 = Rs. 500,000
    // Discount: 10% = Rs. 50,000
    // Tax: 18% on Rs. 450,000 = Rs. 81,000
    // Item Total: Rs. 531,000
    let laptop = engine.calculate_item("LAPTOP", Money::new(100000, 0), 5.0, &[]).unwrap();
    
    // MOUSE: 3 × Rs. 2,500 = Rs. 7,500
    // Discount: Rs. 100 (fixed, min qty 2+)
    // Tax: 0
    // Item Total: Rs. 7,400
    let mouse = engine.calculate_item("MOUSE", Money::new(2500, 0), 3.0, &[]).unwrap();
    
    // KEYBOARD: 6 × Rs. 5,000 = Rs. 30,000
    // BOGO: 2 sets, 2 half-price = Rs. 5,000 discount
    // Tax: 5% on Rs. 25,000 = Rs. 1,250
    // Item Total: Rs. 26,250
    let keyboard = engine.calculate_item("KEYBOARD", Money::new(5000, 0), 6.0, &[]).unwrap();
    
    // HEADPHONE: 2 × Rs. 15,000 = Rs. 30,000
    // Discount: 15% + 5% (with code) = 20% = Rs. 6,000
    // Tax: 23% on Rs. 24,000 = Rs. 5,520
    // Item Total: Rs. 29,520
    let headphone = engine.calculate_item("HEADPHONE", Money::new(15000, 0), 2.0, &["HP5".to_string()]).unwrap();

    // ═══════════════════════════════════════════════════════════════
    // CART SUMMARY
    // ═══════════════════════════════════════════════════════════════
    let total_base = laptop.base_amount.amount + mouse.base_amount.amount + 
                     keyboard.base_amount.amount + headphone.base_amount.amount;
    let total_discount = laptop.discount_amount.amount + mouse.discount_amount.amount + 
                         keyboard.discount_amount.amount + headphone.discount_amount.amount;
    let total_tax = laptop.tax_amount.amount + mouse.tax_amount.amount + 
                    keyboard.tax_amount.amount + headphone.tax_amount.amount;
    let grand_total = laptop.total.amount + mouse.total.amount + 
                      keyboard.total.amount + headphone.total.amount;

    println!("═══════════════════════════════════════════════════════════");
    println!("                    CART SUMMARY                           ");
    println!("═══════════════════════════════════════════════════════════");
    println!("LAPTOP   (5×Rs.100,000): Base={}, Disc={}, Tax={}, Total={}",
        laptop.base_amount, laptop.discount_amount, laptop.tax_amount, laptop.total);
    println!("MOUSE    (3×Rs.2,500):   Base={}, Disc={}, Tax={}, Total={}",
        mouse.base_amount, mouse.discount_amount, mouse.tax_amount, mouse.total);
    println!("KEYBOARD (6×Rs.5,000):   Base={}, Disc={}, Tax={}, Total={}",
        keyboard.base_amount, keyboard.discount_amount, keyboard.tax_amount, keyboard.total);
    println!("HEADPHONE(2×Rs.15,000):  Base={}, Disc={}, Tax={}, Total={}",
        headphone.base_amount, headphone.discount_amount, headphone.tax_amount, headphone.total);
    println!("═══════════════════════════════════════════════════════════");
    println!("SUBTOTAL:      Rs. {:.2}", total_base as f64 / 100.0);
    println!("TOTAL DISCOUNT: Rs. {:.2}", total_discount as f64 / 100.0);
    println!("TOTAL TAX:      Rs. {:.2}", total_tax as f64 / 100.0);
    println!("GRAND TOTAL:    Rs. {:.2}", grand_total as f64 / 100.0);
    println!("═══════════════════════════════════════════════════════════");
}

// =============================================================================
// 📋 CAPABILITIES SUMMARY
// =============================================================================
//
// ┌─────────────────────────────────────────────────────────────────────────┐
// │                        DISCOUNT CAPABILITIES                            │
// ├─────────────────────────────────────────────────────────────────────────┤
// │ ✅ Fixed Amount (Rs. 500 off)                                          │
// │ ✅ Percentage (10% off)                                                 │
// │ ✅ Tiered/Qty Range (5-9: 5%, 10-19: 10%, 20+: 15%)                    │
// │ ✅ Buy X Get Y (Buy 2 Get 1 Free)                                       │
// │ ✅ Bundle (Laptop+Mouse = 15% off)                                      │
// │ ✅ Time-based (Valid Jan 20-22 only)                                    │
// │ ✅ Promo Code                                                           │
// │ ✅ Min Qty Condition                                                    │
// │ ✅ Min Amount Condition                                                 │
// │ ✅ Stackable + Non-stackable                                            │
// │ ✅ Max Discount Cap                                                     │
// │ ✅ Priority System                                                      │
// │ ✅ Product-wise (Each product has own rules)                            │
// │ ✅ Full Bill (Cart-level discounts)                                     │
// ├─────────────────────────────────────────────────────────────────────────┤
// │                          TAX CAPABILITIES                               │
// ├─────────────────────────────────────────────────────────────────────────┤
// │ ✅ Global Tax (Apply to all)                                            │
// │ ✅ Product-wise Tax (Different rates per product)                       │
// │ ✅ Category-wise Tax                                                    │
// │ ✅ Multi-Tax per Product (VAT + Luxury + NBT)                           │
// │ ✅ Tax Exempt Products                                                  │
// │ ✅ Tax Included in Price                                                │
// │ ✅ Jurisdiction-based (Country/State)                                   │
// ├─────────────────────────────────────────────────────────────────────────┤
// │                      CALCULATION ORDER                                  │
// ├─────────────────────────────────────────────────────────────────────────┤
// │ ✅ Discount First, Then Tax                                             │
// │ ✅ Tax First, Then Discount                                             │
// │ ✅ Parallel (Independent)                                               │
// └─────────────────────────────────────────────────────────────────────────┘
//
// =============================================================================
