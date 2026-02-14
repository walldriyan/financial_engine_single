# 🚀 Financial Engine - භාවිත අත්පොත (Usage Guide)

මෙම ලේඛනය **Financial Engine** එක භාවිතා කර සංකීර්ණ ගණනය කිරීම් සිදු කරන ආකාරය පියවරෙන් පියවර විස්තර කරයි.

---

## 🛠️ මූලික සැකසුම (Basic Setup)

එන්ජිම භාවිතා කිරීමට පෙර පහත පියවර අනුගමනය කරන්න.

### 1. Engine එක ආරම්භ කිරීම

```rust
use financial_engine::rules::mixed_scenarios::{MixedScenarioEngine, TaxRate, TaxAppliesTo, ProductDiscountConfig, DiscountRule, DiscountType};
use financial_engine::core::money::Money;

let mut engine = MixedScenarioEngine::new();
```

---

## 🎯 Scenario 1: Ultra-Complex Product (භාණ්ඩ 3ක්, එකකට වට්ටම් 3ක් සහ බදු 2ක්)

මෙහිදී අපි **Laptop** එකක් සඳහා වට්ටම් 3ක් (Seasonal, Loyalty, Promo) සහ බදු 2ක් (VAT, SSCL) එකවර ක්‍රියාත්මක කරමු.

### Rust Code:

```rust
// 1. බදු දෙකක් (Tax) අර්ථ දැක්වීම (VAT 18% + SSCL 2.5%)
engine.add_product_tax(ProductTaxConfig {
    product_id: "LAPTOP_PRO".to_string(),
    tax_rates: vec![
        TaxRate { name: "VAT".to_string(), rate: 18.0, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::Product("LAPTOP_PRO".to_string()) },
        TaxRate { name: "SSCL".to_string(), rate: 2.5, jurisdiction: "LK".to_string(), applies_to: TaxAppliesTo::Product("LAPTOP_PRO".to_string()) },
    ],
    tax_exempt: false,
    tax_included_in_price: false,
});

// 2. වට්ටම් 3ක් (Multi-Discount) අර්ථ දැක්වීම
engine.add_product_discount(ProductDiscountConfig {
    product_id: "LAPTOP_PRO".to_string(),
    stackable: true, // වට්ටම් එකිනෙක එකතු විය හැක
    max_discount_percent: Some(30.0), // උපරිම වට්ටම 30% කට සීමා කරයි
    discounts: vec![
        // Discount 1: Seasonal Offer (10%)
        DiscountRule {
            id: "SEASONAL".to_string(), name: "Avurudu Sale".to_string(),
            discount_type: DiscountType::Percentage(10.0), priority: 1, conditions: vec![], stackable: true,
        },
        // Discount 2: Loyalty (5%)
        DiscountRule {
            id: "LOYALTY".to_string(), name: "Gold Member".to_string(),
            discount_type: DiscountType::Percentage(5.0), priority: 2, conditions: vec![], stackable: true,
        },
        // Discount 3: Credit Card Promo (Fixed Rs. 5000)
        DiscountRule {
            id: "CC_PROMO".to_string(), name: "Visa Day".to_string(),
            discount_type: DiscountType::FixedAmount(500000), priority: 3, conditions: vec![], stackable: true,
        },
    ],
});
```

---

## 🎯 Scenario 2: JSON Input/Output Integration (API සඳහා)

ඔබට Frontend (React/Next.js) සිට JSON එවා, JSON ලබා ගැනීමට අවශ්‍ය නම් මෙම ක්‍රමය භාවිතා කරන්න.

### Input JSON (Request):

```json
{
  "cart": {
    "items": [
      { "id": "IPHONE_15", "price": 450000.00, "quantity": 1 },
      { "id": "AIRPODS", "price": 85000.00, "quantity": 2 }
    ]
  },
  "promo_codes": ["SUMMER2026"],
  "jurisdiction": "LK"
}
```

### Rust Implementation (JSON Processing):

```rust
use serde_json::{json, Value};

// මෙය API Handler එකක් තුළ ලිවිය හැක
fn handle_calculation(json_input: &str) -> String {
    let request: CalculationRequest = serde_json::from_str(json_input).unwrap();
    let mut engine = MixedScenarioEngine::new();
    
    // ... (Rules Setup here) ...

    let cart = request.to_cart(); // Convert JSON to Engine Cart
    let result = engine.calculate_cart(&cart, &request.promo_codes, Some("LK")).unwrap();

    // Serialize Output to JSON
    serde_json::to_string_pretty(&result).unwrap()
}
```

### Output JSON (Result):

```json
{
  "items": [
    {
      "item_id": "IPHONE_15",
      "base_amount": 45000000,
      "discount_amount": 4500000,
      "tax_amount": 8100000,
      "total": 48600000,
      "discount_details": [
        { "name": "Summer Promo", "amount": 4500000 }
      ],
      "tax_details": [
        { "name": "VAT", "rate": 18.0, "amount": 8100000 }
      ]
    }
  ],
  "subtotal": 62000000,
  "total_discount": 4500000,
  "total_tax": 11160000,
  "grand_total": 68660000
}
```

---

## 🎯 Scenario 3: The "Ultimate" Mix (Bundle + Jurisdiction)

මෙය Amazon මට්ටමේ සංකීර්ණ අවස්ථාවකි.
*   **Item 1 & 2:** Camera + Lens ගත්තොත් "Bundle Discount" එකක් (15% OFF).
*   **Jurisdiction:** ගැනුම්කරු "US" නම් 8% Tax, "LK" නම් 18% Tax.

### Rust Code:

```rust
// Bundle Rule
engine.add_product_discount(ProductDiscountConfig {
    product_id: "CAMERA_BODY".to_string(),
    stackable: false,
    max_discount_percent: None,
    discounts: vec![
        DiscountRule {
            id: "BUNDLE_CAM_LENS".to_string(),
            name: "Lens Bundle Offer".to_string(),
            // කැමරාව සහ ලෙන්ස් එක දෙකම Cart එකේ තිබේ නම් 15% අඩු වේ
            discount_type: DiscountType::Bundle { 
                items: vec!["LENS_50MM".to_string()], // අනෙක් අයිතමය
                discount_percent: 15.0 
            },
            priority: 10,
            conditions: vec![DiscountCondition::CartContains("LENS_50MM".to_string())], // Check if Lens exists
            stackable: false,
        }
    ],
});

// Jurisdiction Tax
engine.add_global_tax(TaxRate {
    name: "Sales Tax US".to_string(),
    rate: 8.0,
    jurisdiction: "US".to_string(), // ඇමරිකාවට පමණයි
    applies_to: TaxAppliesTo::All,
});

engine.add_global_tax(TaxRate {
    name: "VAT LK".to_string(),
    rate: 18.0,
    jurisdiction: "LK".to_string(), // ලංකාවට පමණයි
    applies_to: TaxAppliesTo::All,
});

// Calculate for US Customer
let result_us = engine.calculate_cart(&cart, &[], Some("US")).unwrap();
// Result: 8% Tax applied, 18% Ignored.

// Calculate for LK Customer
let result_lk = engine.calculate_cart(&cart, &[], Some("LK")).unwrap();
// Result: 18% Tax applied, 8% Ignored.
```

---

## 📝 සාරාංශය (Conclusion)

මෙම උදාහරණ මගින් පෙනී යන්නේ මෙම **Financial Engine** එක:
1.  **Microservice** එකක් ලෙස JSON Request/Response සමග වැඩ කිරීමට සූදානම් බව.
2.  එකම භාණ්ඩයට වුවද **Multiple Discounts & Taxes** (Stacking) හැසිරවිය හැකි බව.
3.  **Cross-product Rules** (Bundle) සහ **Region-based Rules** (Jurisdiction) වැනි Enterprise Logic සඳහා සහාය දක්වන බවයි.

දැන් ඔබට මෙය ඕනෑම පද්ධතියක "Brain" එක (Heart of Calculation) ලෙස භාවිතා කළ හැක.
