use chrono::{Duration, Utc};
use financial_engine::api::rest::*;
use financial_engine::core::money::Money;
use financial_engine::rules::mixed_scenarios::*;
use financial_engine::security::encryption::{DataMasker, HashedField, TransactionSignature};
use financial_engine::security::validator::InputValidator;
use financial_engine::storage::database::*;
use financial_engine::subscription::proration::*;
/// ============================================================================
/// 🧪 Comprehensive Integration Tests - A+ Grade
/// ============================================================================
/// Tests for all engine features at enterprise level.
use financial_engine::*;

/// ============================================================================
/// 💰 Money Tests - Banking Grade Precision
/// ============================================================================

#[test]
fn test_money_no_floating_point_errors() {
    // Classic floating point problem: 0.1 + 0.2 != 0.3
    // With our integer cents, this is always exact!
    let a = Money::from_float(0.1);
    let b = Money::from_float(0.2);
    let sum = a + b;
    let expected = Money::from_float(0.3);

    assert_eq!(sum.amount, expected.amount, "No floating point errors!");
}

#[test]
fn test_money_split_with_remainder() {
    // Rs. 100.00 split into 3 parts
    let total = Money::new(100, 0);
    let parts = total.split(3).unwrap();

    // 33.33 + 33.33 + 33.34 = 100.00 (remainder goes to last)
    let recombined: i64 = parts.iter().map(|p| p.amount).sum();
    assert_eq!(recombined, total.amount, "Split must preserve total!");
}

#[test]
fn test_money_percentage_operations() {
    let price = Money::new(100, 0);

    // Add 10%
    let with_markup = price.add_percentage(10.0);
    assert_eq!(with_markup.amount, 11000); // Rs. 110.00

    // Subtract 20%
    let discounted = price.sub_percentage(20.0);
    assert_eq!(discounted.amount, 8000); // Rs. 80.00
}

/// ============================================================================
/// 🎯 Mixed Tax/Discount Tests - Amazon/eBay Grade
/// ============================================================================

#[test]
fn test_product_specific_tax_rates() {
    let mut engine = MixedScenarioEngine::new();

    // Add different tax rates for different products
    engine.add_product_tax(ProductTaxConfig {
        product_id: "FOOD001".to_string(),
        tax_rates: vec![TaxRate {
            name: "Food Tax".to_string(),
            rate: 0.0, // Food is tax-free
            jurisdiction: "LK".to_string(),
            applies_to: TaxAppliesTo::Product("FOOD001".to_string()),
        }],
        tax_exempt: true,
        tax_included_in_price: false,
    });

    engine.add_product_tax(ProductTaxConfig {
        product_id: "LUXURY001".to_string(),
        tax_rates: vec![
            TaxRate {
                name: "VAT".to_string(),
                rate: 18.0, // 18% VAT
                jurisdiction: "LK".to_string(),
                applies_to: TaxAppliesTo::All,
            },
            TaxRate {
                name: "Luxury Tax".to_string(),
                rate: 5.0, // Additional 5% luxury tax
                jurisdiction: "LK".to_string(),
                applies_to: TaxAppliesTo::All,
            },
        ],
        tax_exempt: false,
        tax_included_in_price: false,
    });

    // Food item - should have 0 tax
    let food_result = engine
        .calculate_item("FOOD001", Money::new(100, 0), 1.0, &[])
        .unwrap();
    assert_eq!(food_result.tax_amount.amount, 0);

    // Luxury item - should have 23% tax (18% + 5%)
    let luxury_result = engine
        .calculate_item("LUXURY001", Money::new(100, 0), 1.0, &[])
        .unwrap();
    assert_eq!(luxury_result.tax_amount.amount, 2300); // Rs. 23.00
}

#[test]
fn test_tiered_quantity_discount() {
    let mut engine = MixedScenarioEngine::new();

    engine.add_product_discount(ProductDiscountConfig {
        product_id: "BULK001".to_string(),
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
                    max_qty: Some(99.0),
                    discount_percent: 10.0,
                },
                TierLevel {
                    min_qty: 100.0,
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

    // Buy 50 items - should get 10% discount
    let result = engine
        .calculate_item(
            "BULK001",
            Money::new(10, 0), // Rs. 10 each
            50.0,
            &[],
        )
        .unwrap();

    // 50 * Rs.10 = Rs.500, 10% off = Rs.50 discount
    assert_eq!(result.base_amount.amount, 50000);
    assert_eq!(result.discount_amount.amount, 5000);
}

#[test]
fn test_buy_x_get_y_free() {
    let mut engine = MixedScenarioEngine::new();

    engine.add_product_discount(ProductDiscountConfig {
        product_id: "BOGO001".to_string(),
        discounts: vec![DiscountRule {
            id: "BOGO".to_string(),
            name: "Buy 2 Get 1 Free".to_string(),
            discount_type: DiscountType::BuyXGetY {
                buy: 2.0,
                get: 1.0,
                free_percent: 100.0,
            },
            priority: 1,
            conditions: vec![],
            stackable: false,
        }],
        stackable: false,
        max_discount_percent: None,
    });

    // Buy 6 items - should get 2 free (2 complete sets of buy 2 get 1)
    let result = engine
        .calculate_item(
            "BOGO001",
            Money::new(100, 0), // Rs. 100 each
            6.0,
            &[],
        )
        .unwrap();

    assert_eq!(result.base_amount.amount, 60000); // Rs. 600
    assert_eq!(result.discount_amount.amount, 20000); // Rs. 200 (2 free items)
}

#[test]
fn test_promo_code_discount() {
    let mut engine = MixedScenarioEngine::new();

    engine.add_product_discount(ProductDiscountConfig {
        product_id: "PROMO001".to_string(),
        discounts: vec![DiscountRule {
            id: "PROMO".to_string(),
            name: "VIP Discount".to_string(),
            discount_type: DiscountType::Percentage(25.0),
            priority: 1,
            conditions: vec![DiscountCondition::PromoCode("VIP25".to_string())],
            stackable: true,
        }],
        stackable: true,
        max_discount_percent: Some(50.0),
    });

    // Without promo code - no discount
    let result_no_code = engine
        .calculate_item("PROMO001", Money::new(100, 0), 1.0, &[])
        .unwrap();
    assert_eq!(result_no_code.discount_amount.amount, 0);

    // With promo code - 25% discount
    let result_with_code = engine
        .calculate_item("PROMO001", Money::new(100, 0), 1.0, &["VIP25".to_string()])
        .unwrap();
    assert_eq!(result_with_code.discount_amount.amount, 2500);
}

/// ============================================================================
/// 🛡️ Security Tests - OWASP Compliant
/// ============================================================================

#[test]
fn test_sql_injection_prevention() {
    let malicious_input = "'; DROP TABLE users; --";
    let result = InputValidator::check_sql_injection(malicious_input);
    assert!(result.is_err(), "SQL injection should be detected!");
}

#[test]
fn test_xss_prevention() {
    let xss_input = "<script>alert('xss')</script>";
    let result = InputValidator::check_xss(xss_input);
    assert!(result.is_err(), "XSS attack should be detected!");
}

#[test]
fn test_credit_card_luhn_validation() {
    // Valid test card number (Visa)
    let valid_card = "4111111111111111";
    assert!(InputValidator::validate_card_luhn(valid_card).unwrap());

    // Invalid card number
    let invalid_card = "4111111111111112";
    assert!(!InputValidator::validate_card_luhn(invalid_card).unwrap());
}

#[test]
fn test_data_masking() {
    let card = "4111222233334444";
    let masked = DataMasker::mask_card(card);
    assert_eq!(masked, "****-****-****-4444");

    let email = "user@example.com";
    let masked_email = DataMasker::mask_email(email);
    assert_eq!(masked_email, "u***@example.com");
}

#[test]
fn test_transaction_signature() {
    let secret = "super_secret_key_123";
    let sig = TransactionSignature::sign("TXN001", 10000, secret);

    // Valid verification
    assert!(sig.verify(10000, secret));

    // Tampered amount should fail
    assert!(!sig.verify(10001, secret));

    // Wrong key should fail
    assert!(!sig.verify(10000, "wrong_key"));
}

#[test]
fn test_password_hashing() {
    let password = "MySecurePassword123!";
    let salt = "random_salt_xyz";

    let hashed = HashedField::new(password, salt);

    // Correct password verifies
    assert!(hashed.verify(password));

    // Wrong password fails
    assert!(!hashed.verify("WrongPassword"));
}

/// ============================================================================
/// 📅 Subscription Proration Tests
/// ============================================================================

#[test]
fn test_mid_cycle_upgrade_proration() {
    let now = Utc::now();
    let request = ProrationRequest {
        subscription_id: "SUB001".to_string(),
        old_plan_amount: Money::new(100, 0), // Rs. 100/month
        new_plan_amount: Money::new(200, 0), // Rs. 200/month
        billing_cycle_start: now - Duration::days(15),
        billing_cycle_end: now + Duration::days(15),
        change_date: now,
        proration_method: ProrationMethod::DayBased,
    };

    let result = ProrationEngine::calculate(&request).unwrap();

    // 50% through cycle
    // Credit from old: Rs. 50
    // Charge for new: Rs. 100
    // Net: Rs. 50 to charge
    assert!(result.net_amount.is_positive());
    assert!((result.proration_factor - 0.5).abs() < 0.1);
}

#[test]
fn test_usage_based_billing() {
    let result = ProrationEngine::usage_based(
        Money::new(100, 0),    // Base: Rs. 100
        1000.0,                // Included: 1000 API calls
        1500.0,                // Actual: 1500 API calls
        Money::from_cents(10), // Overage: Rs. 0.10 per call
    )
    .unwrap();

    // Overage: 500 calls * Rs. 0.10 = Rs. 50
    assert_eq!(result.overage_units, 500.0);
    assert_eq!(result.overage_charge.amount, 5000); // Rs. 50
    assert_eq!(result.total_charge.amount, 15000); // Rs. 150
}

#[test]
fn test_cancellation_refund() {
    let now = Utc::now();
    let result = ProrationEngine::cancellation_refund(
        Money::new(300, 0), // Rs. 300/month
        now - Duration::days(10),
        now + Duration::days(20),
        now,
        RefundPolicy::Prorated,
    )
    .unwrap();

    // Used 10 days, 20 remaining out of 30
    // Refund: Rs. 300 * (20/30) = Rs. 200
    assert_eq!(result.days_used, 10);
    assert_eq!(result.days_unused, 20);
    assert!(result.refund_amount.amount > 0);
}

/// ============================================================================
/// 💾 Storage Tests - Any Database
/// ============================================================================

#[test]
fn test_in_memory_storage() {
    let storage = InMemoryStorage::new();

    // Set
    storage.set("user:123", r#"{"name": "John"}"#).unwrap();

    // Get
    let value = storage.get("user:123").unwrap();
    assert!(value.is_some());
    assert!(value.unwrap().contains("John"));

    // Exists
    assert!(storage.exists("user:123").unwrap());

    // Delete
    storage.delete("user:123").unwrap();
    assert!(!storage.exists("user:123").unwrap());
}

#[test]
fn test_money_json_serialization() {
    let money = Money::new(123, 45);
    let json = EntitySerializer::to_json(&money).unwrap();

    // Deserialize back
    let parsed: Money = EntitySerializer::from_json(&json).unwrap();
    assert_eq!(parsed.amount, money.amount);
}

#[test]
fn test_database_config() {
    let config = DatabaseConfig::postgres_default();
    let conn = config.connection_string();

    assert!(conn.starts_with("postgres://"));
    assert!(conn.contains("5432"));
}

/// ============================================================================
/// 🌐 API Tests - REST/GraphQL Ready
/// ============================================================================

#[test]
fn test_api_request_response() {
    let request = ApiRequest::new(CalculationRequest {
        items: vec![ItemInput {
            id: "ITEM001".to_string(),
            name: "Test Product".to_string(),
            price: 100.0,
            quantity: 2.0,
            category: None,
            tax_class: None,
            discount_eligible: true,
        }],
        customer_id: Some("CUST001".to_string()),
        discount_codes: vec!["SAVE10".to_string()],
        tax_region: Some("LK".to_string()),
        currency: "LKR".to_string(),
    })
    .with_auth("Bearer token123")
    .with_client("client456");

    assert!(request.auth_token.is_some());
    assert!(request.client_id.is_some());
    assert!(!request.request_id.is_empty());
}

#[test]
fn test_api_response_success() {
    let response: ApiResponse<String> =
        ApiResponse::success("req-123", "Calculation complete".to_string(), 50);

    assert!(response.success);
    assert!(response.error.is_none());
    assert_eq!(response.duration_ms, 50);
}

#[test]
fn test_api_response_error() {
    let response: ApiResponse<String> =
        ApiResponse::error("req-456", "VALIDATION_ERROR", "Invalid amount");

    assert!(!response.success);
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, "VALIDATION_ERROR");
}

#[test]
fn test_money_dto_conversion() {
    let money = Money::new(250, 75);
    let dto: MoneyDto = money.into();

    assert_eq!(dto.amount, 25075);
    assert!(dto.formatted.contains("250.75"));
    assert_eq!(dto.currency, "LKR");
}

/// ============================================================================
/// 🏦 Integration Test - Full E-commerce Flow
/// ============================================================================

#[test]
fn test_full_ecommerce_transaction() {
    let mut engine = MixedScenarioEngine::new();

    // Setup taxes
    engine.add_global_tax(TaxRate {
        name: "VAT".to_string(),
        rate: 12.0,
        jurisdiction: "LK".to_string(),
        applies_to: TaxAppliesTo::All,
    });

    // Setup discounts
    engine.add_product_discount(ProductDiscountConfig {
        product_id: "LAPTOP001".to_string(),
        discounts: vec![DiscountRule {
            id: "BULK".to_string(),
            name: "Bulk Discount".to_string(),
            discount_type: DiscountType::Tiered(vec![TierLevel {
                min_qty: 5.0,
                max_qty: None,
                discount_percent: 10.0,
            }]),
            priority: 1,
            conditions: vec![],
            stackable: true,
        }],
        stackable: true,
        max_discount_percent: Some(30.0),
    });

    // Simulate cart with 5 laptops at Rs. 100,000 each
    let result = engine
        .calculate_item("LAPTOP001", Money::new(100_000, 0), 5.0, &[])
        .unwrap();

    // Base: 5 * Rs.100,000 = Rs.500,000
    // Discount: 10% = Rs.50,000
    // After discount: Rs.450,000
    // Tax: 12% = Rs.54,000
    // Total: Rs.504,000

    assert_eq!(result.base_amount.amount, 50_000_000); // 500,000 in cents
    assert_eq!(result.discount_amount.amount, 5_000_000); // 50,000 discount
    assert!(result.total.amount > result.base_amount.amount - result.discount_amount.amount);
}
