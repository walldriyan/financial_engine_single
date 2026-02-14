use serde::{Deserialize, Serialize};
use crate::core::money::Money;
use crate::core::errors::{EngineResult, EngineError};

/// ============================================================================
/// 🌐 REST/GraphQL API Interface (API අතුරුමුහුණත)
/// ============================================================================
/// Universal API layer compatible with:
/// - Next.js, Prisma, GraphQL
/// - Flutter, React Native, iOS Swift
/// - Any REST/JSON consumer

/// 📥 API Request Wrapper (ඉල්ලීම් එතුම)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest<T> {
    pub request_id: String,
    pub timestamp: i64,
    pub version: String,
    pub auth_token: Option<String>,
    pub client_id: Option<String>,
    pub payload: T,
}

impl<T> ApiRequest<T> {
    pub fn new(payload: T) -> Self {
        ApiRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            version: "1.0.0".to_string(),
            auth_token: None,
            client_id: None,
            payload,
        }
    }

    pub fn with_auth(mut self, token: &str) -> Self {
        self.auth_token = Some(token.to_string());
        self
    }

    pub fn with_client(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_string());
        self
    }
}

/// 📤 API Response Wrapper (ප්‍රතිචාර එතුම)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub request_id: String,
    pub timestamp: i64,
    pub duration_ms: i64,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i64,
    pub total_pages: i32,
}

impl<T> ApiResponse<T> {
    pub fn success(request_id: &str, data: T, duration_ms: i64) -> Self {
        ApiResponse {
            success: true,
            request_id: request_id.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            duration_ms,
            data: Some(data),
            error: None,
            pagination: None,
        }
    }

    pub fn error(request_id: &str, code: &str, message: &str) -> ApiResponse<T> {
        ApiResponse {
            success: false,
            request_id: request_id.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            duration_ms: 0,
            data: None,
            error: Some(ApiError {
                code: code.to_string(),
                message: message.to_string(),
                field: None,
                details: None,
            }),
            pagination: None,
        }
    }

    pub fn with_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }
}

/// 💰 Calculation Request (ගණනය කිරීමේ ඉල්ලීම)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationRequest {
    pub items: Vec<ItemInput>,
    pub customer_id: Option<String>,
    pub discount_codes: Vec<String>,
    pub tax_region: Option<String>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInput {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: f64,
    pub category: Option<String>,
    pub tax_class: Option<String>,
    pub discount_eligible: bool,
}

/// 💵 Calculation Response (ගණනය කිරීමේ ප්‍රතිචාරය)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResponse {
    pub subtotal: MoneyDto,
    pub discount_total: MoneyDto,
    pub tax_total: MoneyDto,
    pub grand_total: MoneyDto,
    pub applied_discounts: Vec<AppliedDiscount>,
    pub applied_taxes: Vec<AppliedTax>,
    pub breakdown: Vec<LineItemBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyDto {
    pub amount: i64,          // Cents/smallest unit
    pub formatted: String,    // Display string (Rs. 100.50)
    pub currency: String,
}

impl From<Money> for MoneyDto {
    fn from(money: Money) -> Self {
        MoneyDto {
            amount: money.amount,
            formatted: money.to_string(),
            currency: "LKR".to_string(), // Default, should be configurable
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedDiscount {
    pub code: Option<String>,
    pub name: String,
    pub discount_type: String,
    pub amount: MoneyDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedTax {
    pub name: String,
    pub rate: f64,
    pub amount: MoneyDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemBreakdown {
    pub item_id: String,
    pub item_name: String,
    pub unit_price: MoneyDto,
    pub quantity: f64,
    pub subtotal: MoneyDto,
    pub discount: MoneyDto,
    pub tax: MoneyDto,
    pub total: MoneyDto,
}

/// 🔄 Refund Request (ආපසු ගෙවීමේ ඉල්ලීම)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundApiRequest {
    pub transaction_id: String,
    pub items: Vec<RefundItemInput>,
    pub reason: String,
    pub refund_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemInput {
    pub item_id: String,
    pub quantity: f64,
}

/// 📊 Report Request (වාර්තා ඉල්ලීම)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub report_type: String,
    pub from_date: String,
    pub to_date: String,
    pub filters: Option<serde_json::Value>,
}

/// 🛒 Order Request (ඇණවුම් ඉල්ලීම)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub calculation: CalculationRequest,
    pub customer: CustomerInput,
    pub payment: PaymentInput,
    pub shipping: Option<ShippingInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInput {
    pub id: Option<String>,
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInput {
    pub method: String,
    pub card_token: Option<String>,
    pub billing_address: Option<AddressInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingInput {
    pub method: String,
    pub address: AddressInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInput {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

/// 🔌 API Handler Trait (API හසුරුවන්නා)
pub trait ApiHandler {
    /// Calculate cart totals
    fn calculate(&self, request: CalculationRequest) -> EngineResult<CalculationResponse>;
    
    /// Process refund
    fn refund(&self, request: RefundApiRequest) -> EngineResult<serde_json::Value>;
    
    /// Generate report
    fn report(&self, request: ReportRequest) -> EngineResult<serde_json::Value>;
}

/// 🌐 HTTP Status Codes
pub enum HttpStatus {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    UnprocessableEntity = 422,
    TooManyRequests = 429,
    InternalError = 500,
}

impl From<&EngineError> for HttpStatus {
    fn from(error: &EngineError) -> Self {
        match error {
            EngineError::Validation { .. } => HttpStatus::BadRequest,
            EngineError::NotFound { .. } => HttpStatus::NotFound,
            EngineError::Security { .. } => HttpStatus::Forbidden,
            EngineError::Calculation { .. } => HttpStatus::UnprocessableEntity,
            _ => HttpStatus::InternalError,
        }
    }
}

/// 📋 API Endpoints Definition (API endpoints අර්ථ දැක්වීම)
pub struct ApiEndpoints;

impl ApiEndpoints {
    // Core calculation
    pub const CALCULATE: &'static str = "/api/v1/calculate";
    pub const CALCULATE_BATCH: &'static str = "/api/v1/calculate/batch";
    
    // Orders
    pub const ORDER_CREATE: &'static str = "/api/v1/orders";
    pub const ORDER_GET: &'static str = "/api/v1/orders/:id";
    pub const ORDER_LIST: &'static str = "/api/v1/orders";
    
    // Refunds
    pub const REFUND_CREATE: &'static str = "/api/v1/refunds";
    pub const REFUND_GET: &'static str = "/api/v1/refunds/:id";
    
    // Reports
    pub const REPORT_SALES: &'static str = "/api/v1/reports/sales";
    pub const REPORT_TAX: &'static str = "/api/v1/reports/tax";
    pub const REPORT_INVENTORY: &'static str = "/api/v1/reports/inventory";
    
    // Ledger
    pub const LEDGER_ENTRIES: &'static str = "/api/v1/ledger/entries";
    pub const LEDGER_BALANCE: &'static str = "/api/v1/ledger/balance/:account_id";
    
    // Inventory
    pub const INVENTORY_STOCK: &'static str = "/api/v1/inventory/stock";
    pub const INVENTORY_MOVEMENT: &'static str = "/api/v1/inventory/movements";
    
    // Health & Meta
    pub const HEALTH: &'static str = "/api/v1/health";
    pub const VERSION: &'static str = "/api/v1/version";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_request_creation() {
        let req = ApiRequest::new(CalculationRequest {
            items: vec![],
            customer_id: None,
            discount_codes: vec![],
            tax_region: None,
            currency: "LKR".to_string(),
        })
        .with_auth("token123")
        .with_client("client456");

        assert!(req.auth_token.is_some());
        assert!(req.client_id.is_some());
    }

    #[test]
    fn test_api_response_creation() {
        let response: ApiResponse<String> = ApiResponse::success("req-123", "Hello".to_string(), 50);
        assert!(response.success);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_money_dto_conversion() {
        let money = Money::new(100, 50);
        let dto: MoneyDto = money.into();
        assert_eq!(dto.amount, 10050);
    }
}
