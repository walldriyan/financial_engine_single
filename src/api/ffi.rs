use crate::core::money::Money;
use crate::core::calculation::CalculationResult;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// 🔗 FFI Bindings (බාහිර භාෂා සම්බන්ධතා)
/// ============================================================================
/// Foreign Function Interface for:
/// - Flutter/Dart
/// - iOS Swift
/// - React Native
/// - C/C++
/// - WebAssembly

/// 📦 C-compatible structs for FFI
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMoneyResult {
    pub success: bool,
    pub amount_cents: i64,
    pub error_code: i32,
    pub error_message: [u8; 256],
}

impl CMoneyResult {
    pub fn success(amount: Money) -> Self {
        CMoneyResult {
            success: true,
            amount_cents: amount.amount,
            error_code: 0,
            error_message: [0; 256],
        }
    }

    pub fn error(code: i32, message: &str) -> Self {
        let mut error_message = [0u8; 256];
        let bytes = message.as_bytes();
        let len = bytes.len().min(255);
        error_message[..len].copy_from_slice(&bytes[..len]);
        
        CMoneyResult {
            success: false,
            amount_cents: 0,
            error_code: code,
            error_message,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CCalculationResult {
    pub success: bool,
    pub subtotal_cents: i64,
    pub discount_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub error_code: i32,
}

/// 🌐 JSON String Interface (Universal)
/// Safe way to pass data to/from any language
#[repr(C)]
pub struct JsonResult {
    pub success: bool,
    pub json_ptr: *mut i8,
    pub json_len: usize,
    pub error_code: i32,
}

/// 📱 Flutter/Dart Compatible Interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlutterCalculationRequest {
    pub items: Vec<FlutterItem>,
    pub discount_codes: Vec<String>,
    pub tax_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlutterItem {
    pub id: String,
    pub name: String,
    pub price_cents: i64,
    pub quantity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlutterCalculationResponse {
    pub success: bool,
    pub subtotal_cents: i64,
    pub discount_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub formatted_subtotal: String,
    pub formatted_discount: String,
    pub formatted_tax: String,
    pub formatted_total: String,
    pub error_message: Option<String>,
}

impl From<CalculationResult> for FlutterCalculationResponse {
    fn from(result: CalculationResult) -> Self {
        FlutterCalculationResponse {
            success: true,
            subtotal_cents: result.subtotal.amount,
            discount_cents: result.discount_total.amount,
            tax_cents: result.tax_total.amount,
            total_cents: result.grand_total.amount,
            formatted_subtotal: result.subtotal.to_string(),
            formatted_discount: result.discount_total.to_string(),
            formatted_tax: result.tax_total.to_string(),
            formatted_total: result.grand_total.to_string(),
            error_message: None,
        }
    }
}

/// 🍎 iOS/Swift Compatible Interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftMoneyDTO {
    pub amount_cents: i64,
    pub currency_code: String,
    pub formatted: String,
}

impl From<Money> for SwiftMoneyDTO {
    fn from(money: Money) -> Self {
        SwiftMoneyDTO {
            amount_cents: money.amount,
            currency_code: "LKR".to_string(),
            formatted: money.to_string(),
        }
    }
}

/// 🌐 WebAssembly Interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRequest {
    pub action: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<WasmError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmError {
    pub code: String,
    pub message: String,
}

/// 📋 WASM Actions
pub mod wasm_actions {
    pub const CALCULATE: &str = "calculate";
    pub const ADD_ITEM: &str = "add_item";
    pub const REMOVE_ITEM: &str = "remove_item";
    pub const APPLY_DISCOUNT: &str = "apply_discount";
    pub const GET_TOTAL: &str = "get_total";
    pub const REFUND: &str = "refund";
    pub const CLEAR_CART: &str = "clear_cart";
}

/// 🔧 FFI Helper Functions
pub struct FfiHelpers;

impl FfiHelpers {
    /// Convert Rust String to C-compatible pointer
    pub fn string_to_c_ptr(s: String) -> *mut i8 {
        let c_string = std::ffi::CString::new(s).unwrap();
        c_string.into_raw()
    }

    /// Convert C string pointer back to Rust String
    /// # Safety
    /// The pointer must be valid and null-terminated
    pub unsafe fn c_ptr_to_string(ptr: *const i8) -> Result<String, std::str::Utf8Error> {
        std::ffi::CStr::from_ptr(ptr).to_str().map(|s| s.to_string())
    }

    /// Free a C string allocated by Rust
    /// # Safety
    /// The pointer must have been allocated by string_to_c_ptr
    pub unsafe fn free_c_string(ptr: *mut i8) {
        if !ptr.is_null() {
            let _ = std::ffi::CString::from_raw(ptr);
        }
    }

    /// Serialize to JSON for cross-language communication
    pub fn to_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }

    /// Deserialize from JSON
    pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// 🔌 Platform Bridge Trait
pub trait PlatformBridge {
    /// Calculate and return JSON response
    fn calculate_json(&self, request_json: &str) -> String;
    
    /// Add item and return updated state
    fn add_item_json(&self, item_json: &str) -> String;
    
    /// Apply discount code
    fn apply_discount_json(&self, code: &str) -> String;
    
    /// Get current cart state
    fn get_state_json(&self) -> String;
    
    /// Process refund
    fn refund_json(&self, refund_json: &str) -> String;
}

/// 📦 Dart/Flutter Code Generator
pub struct DartCodeGenerator;

impl DartCodeGenerator {
    /// Generate Dart class for Money
    pub fn money_class() -> &'static str {
        r#"
class Money {
  final int amountCents;
  final String currency;

  Money({required this.amountCents, this.currency = 'LKR'});

  factory Money.fromJson(Map<String, dynamic> json) {
    return Money(
      amountCents: json['amount_cents'] as int,
      currency: json['currency_code'] as String? ?? 'LKR',
    );
  }

  Map<String, dynamic> toJson() => {
    'amount_cents': amountCents,
    'currency_code': currency,
  };

  double get value => amountCents / 100.0;
  
  String get formatted => 'Rs. ${value.toStringAsFixed(2)}';

  Money operator +(Money other) => Money(amountCents: amountCents + other.amountCents);
  Money operator -(Money other) => Money(amountCents: amountCents - other.amountCents);
  Money operator *(int scalar) => Money(amountCents: amountCents * scalar);
}
        "#
    }

    /// Generate Dart class for CalculationResult
    pub fn calculation_result_class() -> &'static str {
        r#"
class CalculationResult {
  final Money subtotal;
  final Money discount;
  final Money tax;
  final Money total;

  CalculationResult({
    required this.subtotal,
    required this.discount,
    required this.tax,
    required this.total,
  });

  factory CalculationResult.fromJson(Map<String, dynamic> json) {
    return CalculationResult(
      subtotal: Money(amountCents: json['subtotal_cents'] as int),
      discount: Money(amountCents: json['discount_cents'] as int),
      tax: Money(amountCents: json['tax_cents'] as int),
      total: Money(amountCents: json['total_cents'] as int),
    );
  }
}
        "#
    }
}

/// 📱 Swift Code Generator
pub struct SwiftCodeGenerator;

impl SwiftCodeGenerator {
    /// Generate Swift struct for Money
    pub fn money_struct() -> &'static str {
        r#"
struct Money: Codable {
    let amountCents: Int64
    let currencyCode: String
    
    init(amountCents: Int64, currencyCode: String = "LKR") {
        self.amountCents = amountCents
        self.currencyCode = currencyCode
    }
    
    var value: Double {
        return Double(amountCents) / 100.0
    }
    
    var formatted: String {
        return String(format: "Rs. %.2f", value)
    }
    
    static func + (lhs: Money, rhs: Money) -> Money {
        return Money(amountCents: lhs.amountCents + rhs.amountCents)
    }
    
    static func - (lhs: Money, rhs: Money) -> Money {
        return Money(amountCents: lhs.amountCents - rhs.amountCents)
    }
}
        "#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flutter_response() {
        let result = CalculationResult {
            subtotal: Money::new(100, 0),
            discount_total: Money::new(10, 0),
            tax_total: Money::new(9, 0),
            grand_total: Money::new(99, 0),
        };

        let flutter_response: FlutterCalculationResponse = result.into();
        assert!(flutter_response.success);
        assert_eq!(flutter_response.subtotal_cents, 10000);
    }

    #[test]
    fn test_json_serialization() {
        let money = Money::new(100, 50);
        let swift_dto: SwiftMoneyDTO = money.into();
        
        let json = FfiHelpers::to_json(&swift_dto).unwrap();
        assert!(json.contains("10050"));
    }
}
