use crate::core::errors::{EngineError, EngineResult};
use crate::core::money::Money;
// HashSet removed

/// ============================================================================
/// 🛡️ Input Validator (ආදාන වලංගු කරන්නා)
/// ============================================================================
/// OWASP-compliant input validation.
/// SQL Injection, XSS, and other attack prevention.

pub struct InputValidator;

impl InputValidator {
    // Dangerous SQL keywords
    const SQL_KEYWORDS: &'static [&'static str] = &[
        "SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "UNION", "ALTER", "CREATE", "TRUNCATE",
        "EXEC", "EXECUTE", "--", "/*", "*/",
    ];

    // XSS patterns
    const XSS_PATTERNS: &'static [&'static str] = &[
        "<script",
        "</script>",
        "javascript:",
        "onerror=",
        "onload=",
        "onclick=",
        "onmouseover=",
        "eval(",
        "document.cookie",
    ];

    /// 🛑 Validate string for SQL Injection
    pub fn check_sql_injection(input: &str) -> EngineResult<()> {
        let upper = input.to_uppercase();
        for keyword in Self::SQL_KEYWORDS {
            if upper.contains(keyword) {
                return Err(EngineError::Security {
                    code: "SQL_INJECTION_DETECTED".to_string(),
                    message: format!("Potential SQL injection detected: {}", keyword),
                });
            }
        }
        Ok(())
    }

    /// 🛑 Validate string for XSS
    pub fn check_xss(input: &str) -> EngineResult<()> {
        let lower = input.to_lowercase();
        for pattern in Self::XSS_PATTERNS {
            if lower.contains(&pattern.to_lowercase()) {
                return Err(EngineError::Security {
                    code: "XSS_DETECTED".to_string(),
                    message: format!("Potential XSS attack detected"),
                });
            }
        }
        Ok(())
    }

    /// ✅ Sanitize all inputs (comprehensive check)
    pub fn sanitize(input: &str) -> EngineResult<String> {
        Self::check_sql_injection(input)?;
        Self::check_xss(input)?;

        // Remove null bytes and control characters
        let sanitized: String = input
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
            .collect();

        Ok(sanitized)
    }

    /// 💰 Validate money amount (must be positive, within limits)
    pub fn validate_money(amount: &Money, max_amount: &Money) -> EngineResult<()> {
        if amount.is_negative() {
            return Err(EngineError::Validation {
                message: "Amount cannot be negative".to_string(),
            });
        }

        if amount > max_amount {
            return Err(EngineError::Validation {
                message: format!("Amount {} exceeds maximum allowed {}", amount, max_amount),
            });
        }

        Ok(())
    }

    /// 🔢 Validate quantity
    pub fn validate_quantity(qty: f64, max: f64) -> EngineResult<()> {
        if qty <= 0.0 {
            return Err(EngineError::Validation {
                message: "Quantity must be positive".to_string(),
            });
        }
        if qty > max {
            return Err(EngineError::Validation {
                message: format!("Quantity {} exceeds maximum {}", qty, max),
            });
        }
        if qty.is_nan() || qty.is_infinite() {
            return Err(EngineError::Validation {
                message: "Invalid quantity value".to_string(),
            });
        }
        Ok(())
    }

    /// 📧 Validate email format
    pub fn validate_email(email: &str) -> EngineResult<()> {
        Self::sanitize(email)?;

        if !email.contains('@') || email.len() < 5 {
            return Err(EngineError::Validation {
                message: "Invalid email format".to_string(),
            });
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(EngineError::Validation {
                message: "Invalid email format".to_string(),
            });
        }

        if !parts[1].contains('.') {
            return Err(EngineError::Validation {
                message: "Invalid email domain".to_string(),
            });
        }

        Ok(())
    }

    /// 💳 Validate credit card (Luhn algorithm)
    pub fn validate_card_luhn(card_number: &str) -> EngineResult<bool> {
        let digits: Vec<u32> = card_number
            .chars()
            .filter(|c| c.is_numeric())
            .filter_map(|c| c.to_digit(10))
            .collect();

        if digits.len() < 13 || digits.len() > 19 {
            return Err(EngineError::Validation {
                message: "Invalid card number length".to_string(),
            });
        }

        let mut sum = 0;
        let _len = digits.len();

        for (i, &digit) in digits.iter().rev().enumerate() {
            if i % 2 == 1 {
                let doubled = digit * 2;
                sum += if doubled > 9 { doubled - 9 } else { doubled };
            } else {
                sum += digit;
            }
        }

        Ok(sum % 10 == 0)
    }

    /// 🆔 Validate UUID format
    pub fn validate_uuid(id: &str) -> EngineResult<()> {
        let clean: String = id.chars().filter(|c| *c != '-').collect();

        if clean.len() != 32 {
            return Err(EngineError::Validation {
                message: "Invalid UUID format".to_string(),
            });
        }

        if !clean.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(EngineError::Validation {
                message: "UUID contains invalid characters".to_string(),
            });
        }

        Ok(())
    }
}

/// 🚦 Rate Limiter (වේග සීමා කරන්නා)
/// DDoS and brute-force attack prevention
pub struct RateLimiter {
    requests: std::collections::HashMap<String, Vec<i64>>,
    max_requests: usize,
    window_seconds: i64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: i64) -> Self {
        RateLimiter {
            requests: std::collections::HashMap::new(),
            max_requests,
            window_seconds,
        }
    }

    /// Check if request is allowed
    pub fn allow(&mut self, client_id: &str) -> EngineResult<bool> {
        let now = chrono::Utc::now().timestamp();
        let cutoff = now - self.window_seconds;

        let timestamps = self
            .requests
            .entry(client_id.to_string())
            .or_insert_with(Vec::new);

        // Remove old timestamps
        timestamps.retain(|&ts| ts > cutoff);

        if timestamps.len() >= self.max_requests {
            return Err(EngineError::Security {
                code: "RATE_LIMIT_EXCEEDED".to_string(),
                message: format!(
                    "Rate limit exceeded. Max {} requests per {} seconds",
                    self.max_requests, self.window_seconds
                ),
            });
        }

        timestamps.push(now);
        Ok(true)
    }

    /// Reset limiter for a client
    pub fn reset(&mut self, client_id: &str) {
        self.requests.remove(client_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_injection_detection() {
        assert!(InputValidator::check_sql_injection("SELECT * FROM users").is_err());
        assert!(InputValidator::check_sql_injection("normal text").is_ok());
    }

    #[test]
    fn test_xss_detection() {
        assert!(InputValidator::check_xss("<script>alert('xss')</script>").is_err());
        assert!(InputValidator::check_xss("normal text").is_ok());
    }

    #[test]
    fn test_luhn_validation() {
        // Valid test card number
        assert!(InputValidator::validate_card_luhn("4111111111111111").unwrap());
        // Invalid card
        assert!(!InputValidator::validate_card_luhn("4111111111111112").unwrap());
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(3, 60);
        assert!(limiter.allow("client1").is_ok());
        assert!(limiter.allow("client1").is_ok());
        assert!(limiter.allow("client1").is_ok());
        assert!(limiter.allow("client1").is_err()); // 4th request should fail
    }
}
