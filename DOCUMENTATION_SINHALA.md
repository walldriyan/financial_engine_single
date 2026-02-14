# 💰 MUDAL GANANA ENGINE - සම්පූර්ණ තාක්ෂණික මාර්ගෝපදේශය

> **Version:** 1.0.0  
> **Language:** Rust  
> **License:** MIT  

---

## 📚 පටුන (Table of Contents)

1. [Engine එක හඳුන්වා දීම](#1-engine-එක-හඳුන්වා-දීම)
2. [Architecture Overview](#2-architecture-overview)
3. [Data Flow - දත්ත ගලා යන ආකාරය](#3-data-flow)
4. [Core Modules - මූලික කොටස්](#4-core-modules)
5. [Money Type - මුදල් ව්‍යුහය](#5-money-type)
6. [Rules Engine - රීති එන්ජිම](#6-rules-engine)
7. [Tax Engine - බදු ගණනය](#7-tax-engine)
8. [Discount Engine - වට්ටම් ගණනය](#8-discount-engine)
9. [Mixed Scenarios - මිශ්‍ර අවස්ථා](#9-mixed-scenarios)
10. [Security Layer - ආරක්ෂාව](#10-security-layer)
11. [API Layer - බාහිර සම්බන්ධතා](#11-api-layer)
12. [Real Examples - සැබෑ උදාහරණ](#12-real-examples)

---

## 1. Engine එක හඳුන්වා දීම

### මොකක්ද මේ Engine එක?

මේ Engine එක **මූල්‍ය ගණනය කිරීම්** (Financial Calculations) සඳහා නිර්මාණය කළ Enterprise-grade library එකකි.

### භාවිතා කළ හැකි තැන්:
- 🛒 **E-commerce** - Amazon, eBay, Daraz වැනි
- 🏦 **Banking** - ගනුදෙනු, ණය, පොලී
- 🏪 **POS Systems** - සාප්පු බිල්පත්
- 📱 **Subscription Apps** - Monthly billing
- 🌍 **Multi-currency** - විවිධ මුදල් ඒකක

### ප්‍රධාන විශේෂාංග:
```
✅ Floating Point Errors නැහැ (Banking Precision)
✅ Multi-Tax (එකම භාණ්ඩයට බදු කිහිපයක්)
✅ Mix Discounts (වට්ටම් combine කිරීම)
✅ Product-wise Rules (භාණ්ඩය අනුව රීති)
✅ Pluggable Architecture (නව රීති එකතු කිරීම)
✅ Any Database (ඕනෑම DB එකක්)
✅ Multi-Platform (Flutter, iOS, Web)
```

---

## 2. Architecture Overview

### 2.1 High-Level Structure

```
┌─────────────────────────────────────────────────────────┐
│                    API LAYER                            │
│  (REST, GraphQL, FFI - Flutter/iOS/WASM)               │
├─────────────────────────────────────────────────────────┤
│                  FACADE (FinancialEngine)               │
│  (සියලුම engine access එක තැනකින්)                      │
├──────────┬──────────┬──────────┬──────────┬────────────┤
│   TAX    │ DISCOUNT │  RULES   │  LEDGER  │ INVENTORY  │
│  Engine  │  Engine  │  Engine  │  Engine  │   Engine   │
├──────────┴──────────┴──────────┴──────────┴────────────┤
│                    CORE LAYER                           │
│  (Money, Calculation, Errors, Rounding)                │
├─────────────────────────────────────────────────────────┤
│                  SECURITY LAYER                         │
│  (Encryption, Validation, Audit, Guard)                │
├─────────────────────────────────────────────────────────┤
│                  STORAGE LAYER                          │
│  (PostgreSQL, MySQL, MongoDB, Redis, JSON)             │
└─────────────────────────────────────────────────────────┘
```

### 2.2 Folder Structure

```
src/
├── lib.rs              # Entry point - සියලු modules export
├── core/               # 🧠 මූලික සංරචක
│   ├── money.rs        # Money type (i64 cents)
│   ├── calculation.rs  # Main calculation pipeline
│   ├── errors.rs       # Error types
│   ├── rounding.rs     # Rounding modes
│   └── logger.rs       # Logging
├── rules/              # 📐 Rules Engine
│   ├── traits.rs       # Rule interface
│   ├── mixed_scenarios.rs # Advanced mix calculations
│   ├── promotions.rs   # BOGO, tiered discounts
│   └── conditions.rs   # Rule conditions
├── tax/                # 🏛️ Tax calculation
├── discount/           # 🎁 Discount types
├── security/           # 🛡️ Security
│   ├── encryption.rs   # SHA-256, signatures
│   ├── validator.rs    # Input validation
│   └── audit_trail.rs  # Audit logging
├── api/                # 🌐 External APIs
│   ├── facade.rs       # Main facade
│   ├── rest.rs         # REST DTOs
│   └── ffi.rs          # FFI bindings
├── storage/            # 💾 Database
├── ledger/             # 📚 Double-entry accounting
├── inventory/          # 📦 Stock management
└── subscription/       # 📅 Subscription billing
```

---

## 3. Data Flow

### 3.1 සරල ගනුදෙනුවක Data Flow

```
[Input]                    [Process]                   [Output]
   │                           │                           │
   ▼                           ▼                           ▼
┌──────────┐            ┌─────────────┐            ┌───────────┐
│  Items   │───────────▶│ Calculation │───────────▶│  Result   │
│  + Qty   │            │   Engine    │            │  Totals   │
│  + Price │            └─────────────┘            └───────────┘
└──────────┘                   │
                               │
            ┌──────────────────┼──────────────────┐
            ▼                  ▼                  ▼
      ┌──────────┐      ┌──────────┐       ┌──────────┐
      │  Rules   │      │   Tax    │       │ Discount │
      │  Engine  │      │  Engine  │       │  Engine  │
      └──────────┘      └──────────┘       └──────────┘
```

### 3.2 Step-by-Step Process

```rust
// STEP 1: Input එක ලැබේ
let cart = Cart {
    items: vec![
        Item { name: "Laptop", price: 100000, qty: 2 },
        Item { name: "Mouse", price: 2500, qty: 5 },
    ]
};

// STEP 2: Subtotal ගණනය
// Laptop: 100000 * 2 = 200000
// Mouse: 2500 * 5 = 12500
// Subtotal = 212500

// STEP 3: Rules Apply කිරීම (Priority order)
// Priority 50: BOGO discount → Rs. 5000
// Priority 40: VIP discount → Rs. 3000
// Priority 30: Tax 12% → Rs. 24540

// STEP 4: Final Calculation
// Total = Subtotal - Discounts + Taxes
// Total = 212500 - 8000 + 24540 = 229040
```

---

## 4. Core Modules

### 4.1 Entry Point (lib.rs)

```rust
// lib.rs - මෙතැනින් සියල්ල පටන් ගන්නේ

// Modules declare කිරීම
pub mod core;      // මූලික ව්‍යුහයන්
pub mod rules;     // රීති
pub mod tax;       // බදු
pub mod discount;  // වට්ටම්
pub mod security;  // ආරක්ෂාව
pub mod api;       // API
pub mod storage;   // Database

// Re-exports - පහසුවෙන් access කිරීමට
pub use core::money::Money;
pub use core::errors::EngineResult;
pub use api::facade::FinancialEngine;
```

### 4.2 භාවිතය (How to Use)

```rust
use financial_engine::{FinancialEngine, Money};

fn main() {
    // Engine එක පණ ගන්වන්න
    let mut engine = FinancialEngine::new();
    
    // Items add කරන්න
    engine
        .add_item("Laptop", 100000.0, 2.0)
        .add_item("Mouse", 2500.0, 5.0);
    
    // ගණනය කරන්න
    let result = engine.calculate().unwrap();
    
    println!("Total: {}", result.grand_total);
    // Output: Total: Rs.229,040.00
}
```

---

## 5. Money Type

### 5.1 ගැටලුව - Floating Point Errors

```rust
// ❌ සාමාන්‍ය float භාවිතයේ ගැටලුව
let a: f64 = 0.1;
let b: f64 = 0.2;
let sum = a + b;
println!("{}", sum); // 0.30000000000000004 ❌ Wrong!

// Banking වලදී මේ error එක පිළිගත නොහැක!
```

### 5.2 විසඳුම - Integer Cents

```rust
// ✅ අපේ විසඳුම - Integer cents
pub struct Money {
    pub amount: i64,  // සත (cents) වලින් ගබඩා
}

// Rs. 100.50 => 10050 (cents)
// Rs. 0.01  => 1 (cent)
// Rs. 1000000.00 => 100000000 (cents)
```

### 5.3 Money Operations

```rust
// 1️⃣ Money සෑදීම (Creation)
let price1 = Money::new(100, 50);       // Rs. 100.50
let price2 = Money::from_cents(10050);  // Rs. 100.50
let price3 = Money::from_float(100.50); // Rs. 100.50 (round)
let zero = Money::zero();               // Rs. 0.00

// 2️⃣ ගණිත කාර්යයන් (Arithmetic)
let a = Money::new(100, 0);  // Rs. 100
let b = Money::new(50, 0);   // Rs. 50

let sum = a + b;             // Rs. 150
let diff = a - b;            // Rs. 50
let product = a * 3;         // Rs. 300
let quotient = a / 2;        // Rs. 50

// 3️⃣ Percentage Operations
let price = Money::new(1000, 0); // Rs. 1000

// 10% එකතු කරන්න
let with_tax = price.add_percentage(10.0);
// Rs. 1000 + Rs. 100 = Rs. 1100

// 20% අඩු කරන්න
let discounted = price.sub_percentage(20.0);
// Rs. 1000 - Rs. 200 = Rs. 800

// 4️⃣ Split (කොටස් කිරීම)
let total = Money::new(100, 0);
let parts = total.split(3).unwrap();
// [Rs. 33.33, Rs. 33.33, Rs. 33.34]
// Remainder (1 cent) අවසාන කොටසට

// 5️⃣ Comparisons
let expensive = Money::new(1000, 0);
let cheap = Money::new(100, 0);

expensive > cheap;        // true
cheap.is_positive();      // true
Money::zero().is_zero();  // true
```

### 5.4 Display Format

```rust
let price = Money::new(12345, 67);
println!("{}", price);  // "Rs.12345.67"

let negative = Money::from_cents(-5000);
println!("{}", negative);  // "-Rs.50.00"
```

---

## 6. Rules Engine

### 6.1 Rule Trait - රීති නිර්වචනය

```rust
// rules/traits.rs

/// ඕනෑම Rule එකකට තිබිය යුතු ලක්ෂණ
pub trait Rule {
    /// රීතියේ නම
    fn name(&self) -> &str;
    
    /// මේ Cart එකට apply කළ හැකිද?
    fn can_apply(&self, cart: &Cart) -> bool;
    
    /// රීතිය apply කරන්න
    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>>;
    
    /// Priority (ඉහළ අගය = මුලින් execute)
    fn priority(&self) -> i32;
}
```

### 6.2 Rule Actions

```rust
pub enum RuleAction {
    /// වට්ටමක් (Discount)
    Discount(Money),
    
    /// බද්දක් (Tax)
    Tax(Money),
    
    /// ගාස්තුවක් (Surcharge/Fee)
    Fee(Money),
    
    /// නොමිලේ භාණ්ඩයක්
    FreeItem { item_id: String, qty: f64 },
}
```

### 6.3 Priority System

```
┌────────────────────────────────────────────────────────┐
│                   RULE PRIORITY                        │
├────────────────────────────────────────────────────────┤
│  Priority 100+  │  Critical rules (System overrides)  │
│  Priority 50-99 │  BOGO, Free items                   │
│  Priority 30-49 │  Percentage discounts               │
│  Priority 10-29 │  Fixed discounts                    │
│  Priority 1-9   │  Low priority (Fallback rules)      │
└────────────────────────────────────────────────────────┘

Execution Order: High → Low (100 → 1)
```

### 6.4 Custom Rule Example

```rust
// ඔබේම Rule එකක් සාදන්න
pub struct LoyaltyDiscount {
    name: String,
    discount_percent: f64,
    min_purchase: Money,
}

impl Rule for LoyaltyDiscount {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn can_apply(&self, cart: &Cart) -> bool {
        // Subtotal >= min_purchase නම් apply කරන්න
        cart.subtotal() >= self.min_purchase
    }
    
    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let subtotal = cart.subtotal();
        let discount = subtotal.sub_percentage(self.discount_percent);
        let discount_amount = subtotal - discount;
        
        Ok(vec![RuleAction::Discount(discount_amount)])
    }
    
    fn priority(&self) -> i32 {
        35  // Medium priority
    }
}

// භාවිතය
let loyalty_rule = LoyaltyDiscount {
    name: "VIP 10% Off".to_string(),
    discount_percent: 10.0,
    min_purchase: Money::new(5000, 0),
};

engine.add_rule(Box::new(loyalty_rule));
```

---

## 7. Tax Engine

### 7.1 Tax Rate Structure

```rust
pub struct TaxRate {
    pub name: String,           // "VAT", "GST"
    pub rate: f64,              // 12.0 (percentage)
    pub jurisdiction: String,   // "LK", "US-CA"
    pub applies_to: TaxAppliesTo,
}

pub enum TaxAppliesTo {
    All,                           // සියලු භාණ්ඩ
    Category(String),              // "Electronics"
    Product(String),               // "PROD001"
    Region(String),                // "Western Province"
}
```

### 7.2 Product-wise Tax

```rust
// Product-specific tax configuration
pub struct ProductTaxConfig {
    pub product_id: String,
    pub tax_rates: Vec<TaxRate>,
    pub tax_exempt: bool,           // බදු රහිතද?
    pub tax_included_in_price: bool, // මිලට බදු ඇතුළත්ද?
}

// Example: Different taxes for different products
let mut engine = MixedScenarioEngine::new();

// 1️⃣ ආහාර - බදු රහිත
engine.add_product_tax(ProductTaxConfig {
    product_id: "FOOD001".to_string(),
    tax_rates: vec![],
    tax_exempt: true,
    tax_included_in_price: false,
});

// 2️⃣ ඉලෙක්ට්‍රොනික - 18% VAT
engine.add_product_tax(ProductTaxConfig {
    product_id: "ELEC001".to_string(),
    tax_rates: vec![
        TaxRate {
            name: "VAT".to_string(),
            rate: 18.0,
            jurisdiction: "LK".to_string(),
            applies_to: TaxAppliesTo::All,
        }
    ],
    tax_exempt: false,
    tax_included_in_price: false,
});

// 3️⃣ සුඛෝපභෝගී - 18% VAT + 5% Luxury Tax
engine.add_product_tax(ProductTaxConfig {
    product_id: "LUXURY001".to_string(),
    tax_rates: vec![
        TaxRate { name: "VAT".to_string(), rate: 18.0, ... },
        TaxRate { name: "Luxury Tax".to_string(), rate: 5.0, ... },
    ],
    tax_exempt: false,
    tax_included_in_price: false,
});
```

### 7.3 Tax Calculation Flow

```
┌────────────────────┐
│   Base Amount      │  Rs. 10,000
│   (මුල් මිල)        │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  Check Tax Config  │  Product ID → Tax Rules
│  (බදු රීති පරීක්ෂා) │
└─────────┬──────────┘
          │
    ┌─────┴─────┐
    ▼           ▼
Tax Exempt?   Apply Taxes
    │           │
    │      ┌────┴────┐
    │      ▼         ▼
    │   VAT 18%   Luxury 5%
    │   Rs. 1800  Rs. 500
    │      │         │
    │      └────┬────┘
    │           ▼
    │     Total Tax
    │     Rs. 2,300
    ▼           │
Rs. 0       Rs. 2,300
```

---

## 8. Discount Engine

### 8.1 Discount Types

```rust
pub enum DiscountType {
    /// Fixed amount (ස්ථාවර මුදලක්)
    /// Rs. 500 off
    FixedAmount(i64),
    
    /// Percentage (ප්‍රතිශතයක්)
    /// 10% off
    Percentage(f64),
    
    /// Buy X Get Y (ගන්න දෙන්නෙ)
    /// Buy 2 Get 1 Free (100% free)
    BuyXGetY { 
        buy: f64, 
        get: f64, 
        free_percent: f64 
    },
    
    /// Tiered (ශ්‍රේණිගත)
    /// 5+ items: 5% off, 10+ items: 10% off
    Tiered(Vec<TierLevel>),
    
    /// Bundle (පැකේජ)
    /// Laptop + Mouse + Keyboard = 15% off
    Bundle { 
        items: Vec<String>, 
        discount_percent: f64 
    },
}
```

### 8.2 Discount Conditions

```rust
pub enum DiscountCondition {
    MinQuantity(f64),           // අවම ප්‍රමාණය
    MinAmount(i64),             // අවම මුදල (cents)
    CustomerGroup(String),      // "VIP", "Gold"
    DateRange { from, to },     // කාල සීමාව
    FirstPurchase,              // පළමු මිලදී ගැනීම
    PromoCode(String),          // කූපන් කේතය
    CartContains(String),       // Cart එකේ item තිබිය යුතුයි
}
```

### 8.3 Stackable vs Non-Stackable

```rust
pub struct DiscountRule {
    pub id: String,
    pub name: String,
    pub discount_type: DiscountType,
    pub priority: i32,
    pub conditions: Vec<DiscountCondition>,
    pub stackable: bool,  // 👈 වැදගත්!
}

// Stackable = true:
// VIP 10% + Promo 5% + Seasonal 3% = 18% total ✅

// Stackable = false:
// VIP 10% (priority 50) applies
// Promo 5% (priority 40) SKIPPED ❌
// Only highest priority non-stackable applies
```

### 8.4 Discount Calculation Flow

```
┌─────────────────────────────────────────────────────────┐
│                   DISCOUNT FLOW                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  1. Get all applicable discount rules                   │
│     │                                                   │
│     ▼                                                   │
│  2. Sort by priority (High → Low)                      │
│     │                                                   │
│     ▼                                                   │
│  3. For each rule:                                      │
│     ├─── Check conditions (MinQty, PromoCode, etc.)    │
│     │                                                   │
│     ├─── If stackable=false AND already applied one:   │
│     │    └─── SKIP                                      │
│     │                                                   │
│     └─── Calculate discount amount                      │
│          └─── Add to total discount                     │
│                                                         │
│  4. Apply max_discount_percent cap (if set)            │
│     │                                                   │
│     ▼                                                   │
│  Return total discount                                  │
└─────────────────────────────────────────────────────────┘
```

---

## 9. Mixed Scenarios

### 9.1 Calculation Order

```rust
pub enum CalculationOrder {
    /// Discount first, then tax
    /// Rs. 100 - 10% = Rs. 90 → Tax 12% = Rs. 100.80
    DiscountFirst,
    
    /// Tax first, then discount
    /// Rs. 100 + Tax 12% = Rs. 112 → -10% = Rs. 100.80
    TaxFirst,
    
    /// Parallel (independent)
    /// Rs. 100, Discount = Rs. 10, Tax = Rs. 12
    /// Total = 100 - 10 + 12 = Rs. 102
    Parallel,
}
```

### 9.2 Full Mixed Example

```rust
let mut engine = MixedScenarioEngine::new();
engine.set_calculation_order(CalculationOrder::DiscountFirst);

// 1️⃣ Global Tax
engine.add_global_tax(TaxRate {
    name: "VAT".to_string(),
    rate: 12.0,
    jurisdiction: "LK".to_string(),
    applies_to: TaxAppliesTo::All,
});

// 2️⃣ Product Discount
engine.add_product_discount(ProductDiscountConfig {
    product_id: "LAPTOP001".to_string(),
    discounts: vec![
        // Tiered discount
        DiscountRule {
            id: "TIER".to_string(),
            name: "Bulk Discount".to_string(),
            discount_type: DiscountType::Tiered(vec![
                TierLevel { min_qty: 5.0, max_qty: Some(9.0), discount_percent: 5.0 },
                TierLevel { min_qty: 10.0, max_qty: None, discount_percent: 10.0 },
            ]),
            priority: 40,
            conditions: vec![],
            stackable: false,
        },
        // Promo code discount
        DiscountRule {
            id: "PROMO".to_string(),
            name: "VIP Code".to_string(),
            discount_type: DiscountType::Percentage(5.0),
            priority: 30,
            conditions: vec![DiscountCondition::PromoCode("VIP5".to_string())],
            stackable: true,
        },
    ],
    stackable: true,
    max_discount_percent: Some(20.0), // Max 20% cap
});

// 3️⃣ Calculate
let result = engine.calculate_item(
    "LAPTOP001",
    Money::new(100_000, 0),  // Rs. 100,000
    10.0,                    // 10 units
    &["VIP5".to_string()],   // With promo code
).unwrap();

// Calculation:
// Base: Rs. 1,000,000 (10 × Rs. 100,000)
// Tier Discount (10%): Rs. 100,000
// Promo Discount (5%): Rs. 50,000 (stackable)
// Total Discount: Rs. 150,000 (15%)
// After Discount: Rs. 850,000
// Tax (12%): Rs. 102,000
// Grand Total: Rs. 952,000
```

---

## 10. Security Layer

### 10.1 Input Validation

```rust
use financial_engine::security::validator::InputValidator;

// SQL Injection check
let user_input = "'; DROP TABLE users; --";
let result = InputValidator::check_sql_injection(user_input);
assert!(result.is_err()); // Blocked! ❌

// XSS check
let xss_input = "<script>alert('xss')</script>";
let result = InputValidator::check_xss(xss_input);
assert!(result.is_err()); // Blocked! ❌

// Safe input
let safe_input = "Laptop Computer";
let result = InputValidator::sanitize(safe_input);
assert!(result.is_ok()); // Allowed ✅

// Credit card validation (Luhn algorithm)
let valid_card = "4111111111111111";
assert!(InputValidator::validate_card_luhn(valid_card).unwrap());

let invalid_card = "1234567890123456";
assert!(!InputValidator::validate_card_luhn(invalid_card).unwrap());
```

### 10.2 Data Masking

```rust
use financial_engine::security::encryption::DataMasker;

// Credit card masking
let card = "4111222233334444";
let masked = DataMasker::mask_card(card);
// Output: ****-****-****-4444

// Email masking
let email = "john.doe@example.com";
let masked = DataMasker::mask_email(email);
// Output: j***@example.com

// Bank account masking
let account = "1234567890123456";
let masked = DataMasker::mask_account(account);
// Output: ********3456
```

### 10.3 Transaction Signatures

```rust
use financial_engine::security::encryption::TransactionSignature;

let secret_key = "your_super_secret_key";
let transaction_id = "TXN-2024-001";
let amount = 100000; // Rs. 1000.00 in cents

// Create signature
let sig = TransactionSignature::sign(transaction_id, amount, secret_key);

// Verify (correct amount)
assert!(sig.verify(amount, secret_key)); // ✅

// Verify (tampered amount)
assert!(!sig.verify(100001, secret_key)); // ❌ Fail!

// Verify (wrong key)
assert!(!sig.verify(amount, "wrong_key")); // ❌ Fail!
```

### 10.4 Rate Limiting

```rust
use financial_engine::security::validator::RateLimiter;

// Max 100 requests per 60 seconds
let mut limiter = RateLimiter::new(100, 60);

// Normal requests
for i in 0..100 {
    assert!(limiter.allow("user123").is_ok());
}

// 101st request - blocked!
assert!(limiter.allow("user123").is_err());
// Error: "Rate limit exceeded. Max 100 requests per 60 seconds"
```

---

## 11. API Layer

### 11.1 REST API Usage

```rust
use financial_engine::api::rest::*;

// Create request
let request = ApiRequest::new(CalculationRequest {
    items: vec![
        ItemInput {
            id: "PROD001".to_string(),
            name: "Laptop".to_string(),
            price: 100000.0,
            quantity: 2.0,
            category: Some("Electronics".to_string()),
            tax_class: Some("standard".to_string()),
            discount_eligible: true,
        }
    ],
    customer_id: Some("CUST001".to_string()),
    discount_codes: vec!["VIP10".to_string()],
    tax_region: Some("LK".to_string()),
    currency: "LKR".to_string(),
})
.with_auth("Bearer eyJhbGci...")
.with_client("client-app-001");

// Process and create response
let response: ApiResponse<CalculationResponse> = ApiResponse::success(
    &request.request_id,
    calculation_result,
    45 // duration_ms
);

// Serialize to JSON
let json = serde_json::to_string(&response).unwrap();
```

### 11.2 Flutter/Dart Integration

```dart
// Dart code (generated from api/ffi.rs)

class Money {
  final int amountCents;
  final String currency;

  Money({required this.amountCents, this.currency = 'LKR'});

  double get value => amountCents / 100.0;
  String get formatted => 'Rs. ${value.toStringAsFixed(2)}';
}

class CalculationResult {
  final Money subtotal;
  final Money discount;
  final Money tax;
  final Money total;
  
  // Use in Flutter
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text('Subtotal: ${subtotal.formatted}'),
        Text('Discount: -${discount.formatted}'),
        Text('Tax: +${tax.formatted}'),
        Text('Total: ${total.formatted}'),
      ],
    );
  }
}
```

---

## 12. Real Examples

### 12.1 Example 1: POS Transaction

```rust
// සාප්පු බිල්පතක් ගණනය කිරීම

let mut engine = FinancialEngine::new();

// Items add කරන්න
engine
    .add_item("Rice 5kg", 1200.0, 2.0)         // Rs. 2,400
    .add_item("Milk 1L", 350.0, 4.0)           // Rs. 1,400
    .add_item("Bread", 120.0, 3.0)             // Rs. 360
    .add_item("Chicken 1kg", 850.0, 1.0);      // Rs. 850

// 5% loyalty discount add කරන්න
let loyalty = PercentageDiscount::new("Loyalty", 5.0);
engine.add_rule(Box::new(loyalty));

let result = engine.calculate().unwrap();

// Output:
// Subtotal:      Rs. 5,010.00
// Discount (5%): Rs. 250.50
// Grand Total:   Rs. 4,759.50
```

### 12.2 Example 2: E-commerce Order

```rust
// Online order with mixed taxes and discounts

let mut engine = MixedScenarioEngine::new();

// Setup taxes
engine.add_global_tax(TaxRate {
    name: "NBT".to_string(),
    rate: 2.0,  // 2% NBT
    jurisdiction: "LK".to_string(),
    applies_to: TaxAppliesTo::All,
});

engine.add_product_tax(ProductTaxConfig {
    product_id: "PHONE001".to_string(),
    tax_rates: vec![
        TaxRate { name: "VAT", rate: 18.0, ... },
        TaxRate { name: "Telecom Levy", rate: 2.5, ... },
    ],
    tax_exempt: false,
    tax_included_in_price: false,
});

// Setup discounts
engine.add_product_discount(ProductDiscountConfig {
    product_id: "PHONE001".to_string(),
    discounts: vec![
        DiscountRule {
            id: "FLASH".to_string(),
            name: "Flash Sale".to_string(),
            discount_type: DiscountType::Percentage(15.0),
            priority: 50,
            conditions: vec![
                DiscountCondition::DateRange {
                    from: "2024-01-20".to_string(),
                    to: "2024-01-22".to_string(),
                }
            ],
            stackable: false,
        }
    ],
    stackable: false,
    max_discount_percent: None,
});

// Calculate
let result = engine.calculate_item(
    "PHONE001",
    Money::new(75_000, 0),  // Rs. 75,000
    1.0,
    &[],
).unwrap();

// Calculation:
// Base:          Rs. 75,000.00
// Flash Sale:    Rs. 11,250.00 (15%)
// After Disc:    Rs. 63,750.00
// VAT (18%):     Rs. 11,475.00
// Telecom (2.5%): Rs. 1,593.75
// NBT (2%):      Rs. 1,275.00
// Grand Total:   Rs. 78,093.75
```

### 12.3 Example 3: Subscription Billing

```rust
// Mid-cycle subscription upgrade

use financial_engine::subscription::proration::*;
use chrono::{Utc, Duration};

let now = Utc::now();

let request = ProrationRequest {
    subscription_id: "SUB-2024-001".to_string(),
    old_plan_amount: Money::new(1000, 0),   // Rs. 1,000/month
    new_plan_amount: Money::new(2500, 0),   // Rs. 2,500/month
    billing_cycle_start: now - Duration::days(10),
    billing_cycle_end: now + Duration::days(20),
    change_date: now,
    proration_method: ProrationMethod::DayBased,
};

let result = ProrationEngine::calculate(&request).unwrap();

// Output:
// Days in cycle:    30
// Days remaining:   20
// Proration factor: 0.667 (20/30)
// 
// Credit (old plan): Rs. 666.67 (1000 × 0.667)
// Charge (new plan): Rs. 1,666.67 (2500 × 0.667)
// Net charge:        Rs. 1,000.00
```

---

## 📌 Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│                    QUICK REFERENCE                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  💰 MONEY                                                   │
│  ──────                                                     │
│  Money::new(100, 50)       → Rs. 100.50                    │
│  Money::from_cents(10050)  → Rs. 100.50                    │
│  money.add_percentage(10)  → +10%                          │
│  money.sub_percentage(10)  → -10%                          │
│  money.split(3)            → 3 equal parts                 │
│                                                             │
│  📐 RULES                                                   │
│  ──────                                                     │
│  Priority 50+  → High (First)                              │
│  Priority 1-49 → Low (Last)                                │
│  can_apply()   → Check conditions                          │
│  apply()       → Return RuleAction                         │
│                                                             │
│  🎁 DISCOUNTS                                               │
│  ──────────                                                 │
│  DiscountType::FixedAmount(5000)     → Rs. 50 off          │
│  DiscountType::Percentage(10.0)      → 10% off             │
│  DiscountType::BuyXGetY{2, 1, 100}   → Buy 2 Get 1 Free    │
│  DiscountType::Tiered(tiers)         → Qty-based           │
│                                                             │
│  🏛️ TAXES                                                   │
│  ──────                                                     │
│  TaxAppliesTo::All          → All products                 │
│  TaxAppliesTo::Product(id)  → Specific product             │
│  TaxAppliesTo::Category(c)  → Category items               │
│  tax_exempt: true           → No tax                       │
│                                                             │
│  🛡️ SECURITY                                                │
│  ────────                                                   │
│  InputValidator::check_sql_injection()                      │
│  InputValidator::check_xss()                                │
│  InputValidator::validate_card_luhn()                       │
│  DataMasker::mask_card()                                    │
│  TransactionSignature::sign()                               │
│                                                             │
│  📅 SUBSCRIPTIONS                                           │
│  ──────────────                                             │
│  ProrationEngine::calculate()        → Mid-cycle           │
│  ProrationEngine::usage_based()      → Overage             │
│  ProrationEngine::cancellation_refund()                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 13. 🎯 Advanced Discount/Tax Scenarios (උසස් සටන්)

### 📋 සියලු Capabilities Summary

| Feature | Support | Example |
|---------|---------|---------|
| Fixed Discount | ✅ | Rs. 500 off |
| Percentage Discount | ✅ | 10% off |
| Tiered (Qty From-To) | ✅ | 5-9: 5%, 10-19: 10% |
| Buy X Get Y Free | ✅ | Buy 2 Get 1 Free |
| Time-based | ✅ | Valid Jan 20-22 |
| Promo Code | ✅ | Code "VIP10" |
| Product-wise Discount | ✅ | Laptop has own rules |
| Full Bill Discount | ✅ | Bill > Rs.5000 = Rs.500 off |
| Stackable Discounts | ✅ | VIP + Promo + Seasonal |
| Max Discount Cap | ✅ | Max 25% off |
| Product-wise Tax | ✅ | Food=0%, Electronics=18% |
| Multi-Tax per Product | ✅ | VAT + Luxury + NBT |
| Tax Exempt | ✅ | Food items |

### 🔢 Example 1: Qty Range (From-To) Tiered Discount

```rust
// 5-9: 5% off, 10-19: 10% off, 20-49: 15%, 50+: 20%
engine.add_product_discount(ProductDiscountConfig {
    product_id: "PROD001".to_string(),
    discounts: vec![DiscountRule {
        id: "TIER".to_string(),
        name: "Qty Tier".to_string(),
        discount_type: DiscountType::Tiered(vec![
            TierLevel { min_qty: 5.0,  max_qty: Some(9.0),  discount_percent: 5.0 },
            TierLevel { min_qty: 10.0, max_qty: Some(19.0), discount_percent: 10.0 },
            TierLevel { min_qty: 20.0, max_qty: Some(49.0), discount_percent: 15.0 },
            TierLevel { min_qty: 50.0, max_qty: None,       discount_percent: 20.0 },
        ]),
        priority: 50,
        conditions: vec![],
        stackable: false,
    }],
    stackable: false,
    max_discount_percent: None,
});

// 15 items at Rs. 100 each = Rs. 1500
// Falls in 10-19 range = 10% off = Rs. 150 discount
```

### 🔢 Example 2: Fixed + Percentage Mix (Product එකම)

```rust
engine.add_product_discount(ProductDiscountConfig {
    product_id: "LAPTOP".to_string(),
    discounts: vec![
        // Rule 1: Fixed Rs. 1000 off (priority 50)
        DiscountRule {
            id: "FIXED".to_string(),
            name: "Rs. 1000 Off".to_string(),
            discount_type: DiscountType::FixedAmount(100000),
            priority: 50,
            conditions: vec![],
            stackable: true,  // ✅ Stack allowed
        },
        // Rule 2: 5% extra (priority 40)
        DiscountRule {
            id: "PERC".to_string(),
            name: "5% Extra".to_string(),
            discount_type: DiscountType::Percentage(5.0),
            priority: 40,
            conditions: vec![],
            stackable: true,  // ✅ Stack allowed
        },
    ],
    stackable: true,
    max_discount_percent: Some(20.0),  // Max 20% cap
});

// Laptop Rs. 50,000
// Fixed: Rs. 1,000
// 5%: Rs. 2,500
// Total Discount: Rs. 3,500 (7%) - under cap ✅
```

### 🔢 Example 3: Multiple Products - එකම Cart එකේ

```rust
let mut engine = MixedScenarioEngine::new();

// Product 1: LAPTOP - Tiered + VAT 18%
engine.add_product_discount(ProductDiscountConfig {
    product_id: "LAPTOP".to_string(),
    discounts: vec![DiscountRule {
        discount_type: DiscountType::Tiered(vec![
            TierLevel { min_qty: 5.0, max_qty: None, discount_percent: 10.0 },
        ]),
        ...
    }],
    ...
});
engine.add_product_tax(ProductTaxConfig {
    product_id: "LAPTOP".to_string(),
    tax_rates: vec![TaxRate { name: "VAT", rate: 18.0, ... }],
    tax_exempt: false,
    ...
});

// Product 2: MOUSE - Fixed Rs. 100 off + NO TAX
engine.add_product_discount(ProductDiscountConfig {
    product_id: "MOUSE".to_string(),
    discounts: vec![DiscountRule {
        discount_type: DiscountType::FixedAmount(10000),
        conditions: vec![DiscountCondition::MinQuantity(2.0)],
        ...
    }],
    ...
});
engine.add_product_tax(ProductTaxConfig {
    product_id: "MOUSE".to_string(),
    tax_exempt: true,  // ✅ No tax
    ...
});

// Product 3: KEYBOARD - BOGO + 5% tax
engine.add_product_discount(ProductDiscountConfig {
    product_id: "KEYBOARD".to_string(),
    discounts: vec![DiscountRule {
        discount_type: DiscountType::BuyXGetY { 
            buy: 2.0, get: 1.0, free_percent: 50.0 
        },
        ...
    }],
    ...
});
engine.add_product_tax(ProductTaxConfig {
    product_id: "KEYBOARD".to_string(),
    tax_rates: vec![TaxRate { rate: 5.0, ... }],
    ...
});

// CART CALCULATION:
let laptop = engine.calculate_item("LAPTOP", Money::new(100000, 0), 5.0, &[]);
let mouse = engine.calculate_item("MOUSE", Money::new(2500, 0), 3.0, &[]);
let keyboard = engine.calculate_item("KEYBOARD", Money::new(5000, 0), 6.0, &[]);

// Each product gets its OWN discount and tax rules!
```

### 🔢 Example 4: Time-Based Discount

```rust
DiscountRule {
    id: "FLASH".to_string(),
    name: "Flash Sale".to_string(),
    discount_type: DiscountType::Percentage(30.0),
    priority: 100,
    conditions: vec![
        DiscountCondition::DateRange {
            from: "2024-01-20".to_string(),
            to: "2024-01-22".to_string(),
        }
    ],
    stackable: false,
}
// Only applies between Jan 20-22!
```

### 🔢 Example 5: Promo Code + Min Amount

```rust
DiscountRule {
    id: "SAVE20".to_string(),
    name: "Save 20%".to_string(),
    discount_type: DiscountType::Percentage(20.0),
    conditions: vec![
        DiscountCondition::PromoCode("SAVE20".to_string()),
        DiscountCondition::MinAmount(300000), // Min Rs. 3000
    ],
    ...
}

// ❌ Without code = No discount
// ❌ With code but Rs. 2000 = No discount
// ✅ With code AND Rs. 5000 = 20% off!
```

### 🔢 Example 6: Multi-Tax Per Product

```rust
// Luxury item: VAT 18% + Luxury Tax 5% + NBT 2% = 25% total
engine.add_product_tax(ProductTaxConfig {
    product_id: "LUXURY001".to_string(),
    tax_rates: vec![
        TaxRate { name: "VAT".to_string(), rate: 18.0, ... },
        TaxRate { name: "Luxury Tax".to_string(), rate: 5.0, ... },
        TaxRate { name: "NBT".to_string(), rate: 2.0, ... },
    ],
    tax_exempt: false,
    tax_included_in_price: false,
});

// Rs. 10,000 item
// VAT: Rs. 1,800
// Luxury: Rs. 500
// NBT: Rs. 200
// Total Tax: Rs. 2,500 (25%)
// Grand Total: Rs. 12,500
```

### 🔢 Example 7: Full Bill Discount

```rust
use crate::rules::promotions::GlobalQtyThreshold;

// Total bill > 10 items = Rs. 1000 off
let bill_discount = GlobalQtyThreshold {
    name: "Big Order Discount".to_string(),
    threshold_qty: 10.0,
    discount_amount: Money::new(1000, 0),
};

engine.add_rule(Box::new(bill_discount));
```

### 🔢 Example 8: Complete Cart - All 4 Types Together

```
┌─────────────────────────────────────────────────────────────────┐
│                    COMPLETE CART EXAMPLE                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  LAPTOP (5 × Rs. 100,000)                                      │
│  ├── Product Discount: 10% (qty >= 5)                          │
│  ├── Tax: 18% VAT                                              │
│  └── Total: Rs. 531,000                                         │
│                                                                 │
│  MOUSE (3 × Rs. 2,500)                                         │
│  ├── Product Discount: Rs. 100 fixed                           │
│  ├── Tax: 0% (exempt)                                          │
│  └── Total: Rs. 7,400                                           │
│                                                                 │
│  KEYBOARD (6 × Rs. 5,000)                                      │
│  ├── Product Discount: BOGO (2 half-price)                     │
│  ├── Tax: 5%                                                   │
│  └── Total: Rs. 26,250                                          │
│                                                                 │
│  HEADPHONE (2 × Rs. 15,000)                                    │
│  ├── Product Discount: 15% + 5% promo                          │
│  ├── Tax: 23% (VAT + Luxury)                                   │
│  └── Total: Rs. 29,520                                          │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  SUBTOTAL:       Rs. 567,500.00                                 │
│  TOTAL DISCOUNT: Rs. 61,100.00                                  │
│  TOTAL TAX:      Rs. 87,770.00                                  │
│  GRAND TOTAL:    Rs. 594,170.00                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 📊 Capabilities Table

```
┌────────────────────────────────────────────────────────────────┐
│          ALL SUPPORTED DISCOUNT/TAX COMBINATIONS               │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ✅ Product-wise Discount (Each product = own rules)          │
│  ✅ Qty-wise Discount (5-9: 5%, 10+: 10%)                      │
│  ✅ Full Bill Qty Discount (10+ items = Rs. 500 off)          │
│  ✅ Full Bill Price Discount (Rs. 5000+ = 5% off)             │
│  ✅ All 4 in Same Cart ✅✅✅                                    │
│                                                                │
│  ✅ Product-wise Tax (Laptop=18%, Food=0%)                     │
│  ✅ Multi-Tax per Product (VAT + Luxury + NBT)                 │
│  ✅ Tax Exempt Products                                        │
│  ✅ Global Tax + Product Tax mix                               │
│                                                                │
│  ✅ Fixed Amount Off                                           │
│  ✅ Percentage Off                                             │
│  ✅ Tiered (From-To qty ranges)                                │
│  ✅ Buy X Get Y (BOGO)                                         │
│  ✅ Bundle Discount                                            │
│  ✅ Time-based (Date range)                                    │
│  ✅ Promo Code                                                 │
│  ✅ Min Qty / Min Amount conditions                            │
│  ✅ Stackable + Non-stackable                                  │
│  ✅ Priority system                                            │
│  ✅ Max discount cap                                           │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

## 🎓 ඉගෙනීමේ මාර්ගය

1. ✅ Money type හොඳින් තේරුම් ගන්න
2. ✅ Rule trait implement කරන්න ඉගෙන ගන්න
3. ✅ Mixed scenarios try කරන්න
4. ✅ API layer integrate කරන්න
5. ✅ Security best practices අනුගමනය කරන්න

---

**🙏 ස්තුතියි! Happy Coding!**

*MUDAL GANANA ENGINE - Enterprise Grade Financial Calculations*
