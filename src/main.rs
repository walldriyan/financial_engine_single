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
/// 3. Port 8080 හි සවන් දී සිටියි.

#[tokio::main]
async fn main() {
    // 🚀 සේවා ආරම්භ කිරීමේ පණිවිඩය
    println!("🚀 Starting Ultimate Financial Engine Microservice...");

    // config: පද්ධතියේ සැකසුම් (Environment variables) ලබා ගැනීම.
    let config = financial_engine::storage::config::get_config();

    // 1. Initialize Sentry: දෝෂ වාර්තා කිරීමේ පද්ධතිය ආරම්භ කිරීම.
    // _sentry_guard: යෙදුම ක්‍රියාත්මක වන තෙක් Sentry සේවාව පවත්වාගෙන යයි.
    let _sentry_guard = financial_engine::audit::sentry::SentryGuard::init(config);

    // 2. Initialize Redis: දත්ත වේගයෙන් ලබා ගැනීමට (Caching) භාවිතා කරන පද්ධතිය.
    // _redis_manager: Redis සම්බන්ධතාවය පාලනය කරයි.
    let _redis_manager = financial_engine::storage::redis::RedisManager::init(config);

    // 3. Initialize Logger: පද්ධතියේ සිදුවන දේවල් සටහන් කිරීම (Logging).
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 4. Initialize Database: දත්ත ගබඩාව (PostgreSQL/Supabase) සමඟ සම්බන්ධ වීම.
    match financial_engine::storage::connector::init_db().await {
        Ok(_) => println!("💾 Database System Initialized."),
        Err(e) => {
            println!(
                "❌ CRITICAL ERROR: Database Initialization Failed -> {:?}",
                e
            );
            // දත්ත ගබඩාව නොමැතිව වුවද පද්ධතිය පවත්වා ගැනීමට ඉඩ ලබා දී ඇත.
        }
    }

    // 5. Build Router: API මාර්ග (Routes) සහ Middleware (ආරක්ෂණ ක්‍රම) සැකසීම.
    // app: සම්පූර්ණ වෙබ් යෙදුමේ ව්‍යුහය.
    let app = create_router()
        // TraceLayer: HTTP ඉල්ලීම් පිළිබඳ තොරතුරු සටහන් කරයි.
        .layer(TraceLayer::new_for_http())
        // TimeoutLayer: ඉල්ලීමක් තත්පර 30කට වඩා ගත වුවහොත් එය නවත්වයි.
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        // secure_guard: අනිසි ඇතුළුවීම් වැළැක්වීමේ ආරක්ෂක පද්ධතිය.
        .route_layer(middleware::from_fn(secure_guard));

    // 6. Define Address: සේවාදායකය ක්‍රියාත්මක වන ලිපිනය සහ Port එක තීරණය කිරීම.
    // port: පරිසර විචල්‍යයන්ගෙන් ලබා ගනී (පෙරනිමිය 8080).
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // listener: TCP සම්බන්ධතා සඳහා සවන් දීමේ මෙවලම.
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("✅ Server listening on http://{}", addr);

    // 7. Start Server: සේවාදායකය සක්‍රීයව ක්‍රියාත්මක කිරීම ආරම්භ කරයි.
    axum::serve(listener, app).await.unwrap();
}
