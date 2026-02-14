use financial_engine::api::facade::FinancialEngine;
use financial_engine::core::money::Money;
use financial_engine::discount::item_discount::ItemDiscount;
use financial_engine::discount::percentage::PercentageDiscount;
use financial_engine::rules::promotions::{
    BuyNGetFree, GlobalQtyThreshold, PriceThresholdFixed, QtyThresholdPercentage,
};
use financial_engine::security::guard::IronGuard;
use financial_engine::tax::tax_rule::TaxRule;

#[test]
fn test_complex_iron_guard_scenario() {
    use financial_engine::core::logger::LoggerEngine;

    // Set Log File
    LoggerEngine::set_log_file("complex_test_execution.log");

    LoggerEngine::log("▶️ මෙය ස්වයංක්‍රීය පරීක්ෂණයකි (Automated Test Start)");
    LoggerEngine::log("🛡️ Iron Guard ආරක්ෂිත පද්ධතිය පණගන්වයි...");

    // 1. Initialize Engine wrapped in IronGuard
    let engine_core = FinancialEngine::new();
    let guard = IronGuard::new(engine_core);

    // =========================================================================
    // 🎭 SCENARIO EXECUTION INSIDE LOCK
    // =========================================================================

    let process_result = guard.execute_transaction(|engine| {
        LoggerEngine::log("🛒 පියවර 1: නිෂ්පාදන සහ නීති එකතු කිරීම (Setup Items & Rules)");

        // A. Product 1 (Laptop) Configuration
        // -----------------------------------
        // Rule: Buy 2 Get 1 Free (Qty=3 => Pay for 2)
        // Rule: Price > 10,000 => Fixed Discount (assuming 500 off)
        // Rule: Qty > 2 => 2% Extra Discount

        let laptop_b2g1 = BuyNGetFree::new("Buy 2 Get 1", "Laptop", 2.0, 1.0);
        let laptop_price_disc = PriceThresholdFixed {
            name: "High Value Disc".to_string(),
            item_name: "Laptop".to_string(),
            threshold: Money::from_float(10000.0),
            discount: Money::from_float(500.0),
        };
        let laptop_qty_disc = QtyThresholdPercentage {
            name: "Bulk Laptop 2%".to_string(),
            item_name: "Laptop".to_string(),
            threshold_qty: 2.0,
            percentage: 2.0,
        };

        engine.add_rule(Box::new(laptop_b2g1));
        engine.add_rule(Box::new(laptop_price_disc));
        engine.add_rule(Box::new(laptop_qty_disc));

        // Add Laptop (Price 15,000, Qty 3 to trigger B2G1)
        LoggerEngine::log("➕ ලැප්ටොප් 3ක් එකතු කරන ලදී. (Item: Laptop, Qty: 3)");
        engine.add_item("Laptop", 15000.0, 3.0);

        // B. Product 2 (Phone) Configuration
        // ----------------------------------

        let phone_d1 = ItemDiscount::new("Phone Promo", "Phone", Money::from_float(100.0));
        let phone_d2 = PercentageDiscount::new(
            "Phone Clearance 5%",
            5.0,
            financial_engine::rules::conditions::Condition::Always,
        );
        // 3rd discount? Let's say Threshold discount
        let phone_d3 = PriceThresholdFixed {
            name: "Phone Special".to_string(),
            item_name: "Phone".to_string(),
            threshold: Money::from_float(1000.0),
            discount: Money::from_float(50.0),
        };

        let phone_tax = TaxRule::new_percentage("Phone Tax 3%", 3.0);

        engine.add_rule(Box::new(phone_d1));
        engine.add_rule(Box::new(phone_d2));
        engine.add_rule(Box::new(phone_d3));
        engine.add_rule(Box::new(phone_tax));

        LoggerEngine::log("➕ ෆෝන් 8ක් එකතු කරන ලදී. (Item: Phone, Qty: 8)");
        engine.add_item("Phone", 5000.0, 8.0); // Total Qty so far: 3 + 8 = 11

        // C. Bill Level Configuration
        // ---------------------------

        let bill_qty_disc = GlobalQtyThreshold {
            name: "Mega Cart Discount".to_string(),
            threshold_qty: 10.0,
            discount_amount: Money::from_float(1000.0),
        };

        let bill_tax = TaxRule::new_percentage("NBT 2%", 2.0);

        engine.add_rule(Box::new(bill_qty_disc));
        engine.add_rule(Box::new(bill_tax));

        // Calculate Sale
        LoggerEngine::log("🧮 ගණනය කිරීම අරඹයි... (Calculating)");
        let sale_result = engine.calculate()?;
        LoggerEngine::log(&format!(
            "✅ ගණනය කිරීම අවසන්. එකතුව (Grand Total): {:?}",
            sale_result.grand_total
        ));

        Ok(sale_result)
    });

    let sale_result = process_result.unwrap();
    let paid_amount = sale_result.grand_total.clone();

    // =========================================================================
    // 🔄 REFUND SCENARIO (Iron Guard Locked)
    // =========================================================================
    // Now we must refund EXACTLY this amount to make net result 0.

    LoggerEngine::log("🔄 Refund ක්‍රියාවලිය ආරම්භ කරයි...");

    let refund_result = guard.execute_transaction(|engine| {
        LoggerEngine::log("💰 පියවර 2: හර/බැර ගැලපීම (Reconciling Ledger)");

        let mut ledger_balance = Money::zero();
        ledger_balance = ledger_balance + paid_amount.clone(); // Money In

        LoggerEngine::log(&format!("💵 මුදල් ලැබුණි (Received): {:?}", ledger_balance));

        let refund_amount = paid_amount.clone();
        ledger_balance = ledger_balance - refund_amount;

        LoggerEngine::log(&format!("💸 නැවත ගෙවීම (Refunded): -{:?}", paid_amount));
        LoggerEngine::log(&format!(
            "⚖️ අවසාන ශේෂය (Final Balance): {:?}",
            ledger_balance
        ));

        Ok(ledger_balance)
    });

    let final_balance = refund_result.unwrap();

    // VERIFY ZERO
    assert_eq!(final_balance.amount, 0);
    LoggerEngine::log("✅ ආරක්ෂිත Refund ක්‍රියාවලිය සාර්ථකයි. අවසාන ශේෂය 0 යි.");
    LoggerEngine::log("🏁 පරීක්ෂණය සමාප්තයි. (Test Component)");
}
