use crate::storage::config::MultiDbConfig;

/// ============================================================================
/// 🚨 Fault-Tolerant Sentry Guard (දෝෂ සොයා ගැනීමේ පද්ධතිය)
/// ============================================================================
/// Sentry සම්බන්ධතාවය ස්වයංක්‍රීයව පරීක්ෂා කරයි.
/// DSN නොමැති නම් හෝ සම්බන්ධ වීමට නොහැකි නම්, එන්ජිම බිඳ වැටෙන්නේ නැත (No Panic).
/// ඒ වෙනුවට එය 'Disabled Mode' එකෙන් ක්‍රියා කරයි.

pub struct SentryGuard {
    _guard: Option<sentry::ClientInitGuard>,
}

impl SentryGuard {
    pub fn init(config: &MultiDbConfig) -> Self {
        match &config.sentry_dsn {
            Some(dsn) => {
                println!("🛡️ Sentry Integration: Connecting...");
                let guard = sentry::init((
                    dsn.clone(),
                    sentry::ClientOptions {
                        release: sentry::release_name!(),
                        ..Default::default()
                    },
                ));

                if guard.is_enabled() {
                    println!("✅ Sentry Integration: ACTIVE");
                    SentryGuard {
                        _guard: Some(guard),
                    }
                } else {
                    println!("⚠️ Sentry Integration: FAILED (Check DSN)");
                    SentryGuard { _guard: None }
                }
            }
            None => {
                println!("ℹ️ Sentry Integration: DISABLED (No DSN provided)");
                SentryGuard { _guard: None }
            }
        }
    }
}
