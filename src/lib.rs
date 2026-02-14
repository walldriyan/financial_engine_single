/// ============================================================================
/// 💰 MUDAL GANANA ENGINE - Ultimate Financial Calculation Engine
/// ============================================================================
/// Banking-grade, enterprise-ready financial engine for:
/// - E-commerce (Amazon, eBay grade)
/// - Banking & Financial Services
/// - POS Systems
/// - Subscription Billing
/// - Multi-currency, Multi-tax, Multi-discount scenarios
/// 
/// 🛡️ Security: OWASP compliant, PCI-DSS ready
/// 📊 Accuracy: No floating point errors (integer cents)
/// 🔌 Pluggable: Custom rules, taxes, discounts
/// 🌐 API: REST, GraphQL, FFI (Flutter/iOS/WASM)
/// 💾 Storage: Any SQL/NoSQL database

pub mod core;
pub mod types;
pub mod rules;
pub mod refund;
pub mod audit;
pub mod tax;
pub mod discount;
pub mod state;
pub mod security;
pub mod plugins;
pub mod storage;
pub mod api;
pub mod ledger;
pub mod accounts; // Centralized Creditor/Debtor Management
pub mod advanced_payments; // POS Split Payments & Cheques
pub mod inventory;
pub mod subscription;

// Re-exports for convenience
pub use core::money::Money;
pub use core::errors::{EngineError, EngineResult};
pub use core::calculation::{CalculationEngine, CalculationResult};
pub use api::facade::FinancialEngine;
pub use security::guard::IronGuard;
pub use rules::traits::{Rule, RuleAction};
// Ready for Financial Transactions
// pub type MudalGananaEngine = FinancialEngine; (Removed)
