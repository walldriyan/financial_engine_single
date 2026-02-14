use crate::audit::logger::{LogLevel, Logger};
use axum::{
    body::Bytes,
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// ============================================================================
/// 🛡️ Secure Gateway (ආරක්ෂක දොරටුව)
/// ============================================================================
/// මෙය Microservice එකේ ප්‍රධාන දොරටුවයි (WAF).
/// සෑම Request එකක්ම මෙතනින් පරීක්ෂා කෙරේ.
/// 1. SQL Injection / XSS Attacks වැළැක්වීම.
/// 2. Rate Limiting (කෙටි කාලයක් තුළ අධික ඉල්ලීම් වැළැක්වීම).
/// 3. Request Logging.

#[derive(Clone)]
pub struct SecurityConfig {
    pub max_requests_per_minute: u32,
    pub block_malicious_ips: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 60, // 1 request per second default
            block_malicious_ips: true,
        }
    }
}

/// 🛡️ Main Middleware Logic
pub async fn secure_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    // 1. Check Method
    if req.method() != Method::POST && req.method() != Method::GET {
        return Err(StatusCode::METHOD_NOT_ALLOWED);
    }

    // 2. Simple WAF Logic (Checking Headers/URI for attacks)
    // Note: Checking Body requires buffering which is heavy, usually done in handler or specialized middleware.
    // Here we check URI and basic headers.
    let uri = req.uri().to_string();
    if is_malicious(&uri) {
        println!("🚨 ALERT: Malicious Payload Detected in URI: {}", uri);
        return Err(StatusCode::FORBIDDEN);
    }

    // 3. Logger Injection (Log the incoming request)
    println!(
        "🛡️ GATEWAY: Request allowed -> {} {}",
        req.method(),
        req.uri()
    );

    // 4. Rate Limiting is handled by Tower Layer in main.rs (more efficient)

    // Pass to next layer
    let response = next.run(req).await;
    Ok(response)
}

/// 🕵️ Check for Hack Patterns (SQLi, XSS, Path Traversal)
fn is_malicious(input: &str) -> bool {
    let patterns = vec![
        "union select",
        "drop table",
        "<script>",
        "alert(",
        "../",
        "exec(",
        "base64_decode",
    ];

    let normalized = input.to_lowercase();
    for pattern in patterns {
        if normalized.contains(pattern) {
            return true;
        }
    }
    false
}
