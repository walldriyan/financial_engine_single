use axum::middleware;
use financial_engine::api::routes::create_router;
use financial_engine::security::gateway::secure_guard;

use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// ============================================================================
/// 🚀 Microservice Entry Point (ප්‍රධාන දොරටුව)
/// ============================================================================
/// මෙය සම්පූර්ණ මූල්‍ය එන්ජිම ක්‍රියාත්මක කරන සේවා කේන්ද්‍රයයි (Server).
/// 1. Middleware (Rate Limit, Security) පූරණය කරයි.
/// 2. Engine එක Initialize කරයි.
/// 3. Port 3000 හි සවන් දී සිටියි.

#[tokio::main]
async fn main() {
    // 🚀 Starting Ultimate Financial Engine Microservice...
    let config = financial_engine::storage::config::get_config();

    // 1. Initialize Sentry (Wait for nothing - Fire and forget)
    // The strict guard ensures errors are reported as long as main runs
    let _sentry_guard = financial_engine::audit::sentry::SentryGuard::init(config);

    // 2. Initialize Redis (Optional Cache Layer)
    let _redis_manager = financial_engine::storage::redis::RedisManager::init(config);

    // 3. Initialize Logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🚀 Starting Ultimate Financial Engine Microservice...");

    // 2. Initialize Database (Universal Connector)
    match financial_engine::storage::connector::init_db().await {
        Ok(_) => println!("💾 Database System Initialized."),
        Err(e) => {
            println!(
                "❌ CRITICAL ERROR: Database Initialization Failed -> {:?}",
                e
            );
            // In strict mode, we might want to panic here using std::process::exit(1);
            // But for now we allow running without DB (e.g. In-Memory Mock)
        }
    }

    // 3. Build our Application with Middleware Stack
    let app = create_router()
        // Add Logging Middleware
        .layer(TraceLayer::new_for_http())
        // Add Timeout (Slowloris protection) - 30 seconds max per request
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        // Add Custom Security Guard (WAF Logic)
        .route_layer(middleware::from_fn(secure_guard));

    // 3. Define Address
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("✅ Server listening on http://{}", addr);

    // 4. Start Server

    axum::serve(listener, app).await.unwrap();
}
