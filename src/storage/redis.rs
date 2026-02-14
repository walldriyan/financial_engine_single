use crate::storage::config::MultiDbConfig;

/// ============================================================================
/// ⚡ Fault-Tolerant Redis Integration (ස්වයංක්‍රීය මතක ගබඩාව)
/// ============================================================================
/// Redis තිබේ නම් වේගය වැඩි වේ (Caching).
/// Redis නොමැති නම් කෙලින්ම Database එකෙන් වැඩ කරයි.
/// කිසිදු දෝෂයකින් එන්ජිම නවතින්නේ නැත.

pub struct RedisManager {
    pub client: Option<redis::Client>,
}

impl RedisManager {
    /// 🚀 Initialize Redis (Safe Connect)
    pub fn init(config: &MultiDbConfig) -> Self {
        match &config.redis_url {
            Some(url) => {
                println!("⚡ Redis: Connecting...");
                match redis::Client::open(url.as_str()) {
                    Ok(client) => {
                        println!("✅ Redis Integration: ACTIVE");
                        RedisManager {
                            client: Some(client),
                        }
                    }
                    Err(_) => {
                        println!("⚠️ Redis Connection FAILED: Continuing without Cache.");
                        RedisManager { client: None }
                    }
                }
            }
            None => {
                println!("ℹ️ Redis Integration: DISABLED (No URL provided)");
                RedisManager { client: None }
            }
        }
    }

    /// 📝 Set Value (Safe Set)
    /// Redis නැත්නම් කිසිවක් නොකරයි (No-op)
    pub fn set(&self, key: &str, value: &str) {
        if let Some(client) = &self.client {
            if let Ok(mut con) = client.get_connection() {
                let _: () = redis::cmd("SET")
                    .arg(key)
                    .arg(value)
                    .query(&mut con)
                    .unwrap_or(());
            }
        }
    }

    /// 🔍 Get Value (Safe Get)
    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(client) = &self.client {
            if let Ok(mut con) = client.get_connection() {
                let res: Option<String> =
                    redis::cmd("GET").arg(key).query(&mut con).unwrap_or(None);
                return res;
            }
        }
        None
    }
}
