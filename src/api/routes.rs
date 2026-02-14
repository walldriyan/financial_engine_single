use crate::refund::processor::RefundProcessor;
use crate::refund::types::RefundRequest;
use crate::rules::mixed_scenarios::{CartCalculation, MixedScenarioEngine};
use crate::types::cart::Cart;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json as AxumJson, Router,
};
use serde::Deserialize;
use std::sync::Arc;

/// ============================================================================
/// 🌐 API Routing (API මංපෙත්)
/// ============================================================================
/// මෙය පිටත ලෝකයට `/calculate` සහ `/refund` endpoints විවෘත කරයි.
/// JSON input එකක් ගෙන එය Rust struct එකකට හරවා, එන්ජිමට යවයි.

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<MixedScenarioEngine>,
    pub refund_processor: Arc<RefundProcessor>,
}

/// 📋 Calculate Request DTO
#[derive(Deserialize)]
pub struct CalculateRequest {
    pub cart: Cart,
    pub promo_codes: Vec<String>,
    pub jurisdiction: Option<String>,
}

/// 📋 Refund Request DTO
#[derive(Deserialize)]
pub struct ApiRefundRequest {
    pub original_cart: Cart,
    pub original_calculation: CartCalculation,
    pub refund_request: RefundRequest,
}

// --- Handlers ---

/// 🧮 Calculate Endpoint
async fn calculate_handler(
    State(state): State<AppState>,
    Json(payload): Json<CalculateRequest>,
) -> impl IntoResponse {
    // Engine Logic (Calculate)
    match state.engine.calculate_cart(
        &payload.cart,
        &payload.promo_codes,
        payload.jurisdiction.as_deref(),
    ) {
        Ok(result) => (StatusCode::OK, AxumJson(result)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Error: {:?}", e)).into_response(),
    }
}

/// 🔄 Refund Endpoint
async fn refund_handler(
    State(state): State<AppState>,
    Json(payload): Json<ApiRefundRequest>,
) -> impl IntoResponse {
    // Refund Logic (Reverse Calculation)
    match state.refund_processor.process(
        &payload.original_cart,
        &payload.original_calculation,
        &payload.refund_request,
    ) {
        Ok(result) => (StatusCode::OK, AxumJson(result)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Error: {:?}", e)).into_response(),
    }
}

/// 🛠️ Setup Routes (Router සාදන්න)
pub fn create_router() -> Router {
    // Initialize Engine & Services
    let engine = Arc::new(MixedScenarioEngine::new());
    let refund_processor = Arc::new(RefundProcessor::new());

    let state = AppState {
        engine,
        refund_processor,
    };

    Router::new()
        .route("/api/v1/calculate", post(calculate_handler))
        .route("/api/v1/refund", post(refund_handler))
        .with_state(state)
}
