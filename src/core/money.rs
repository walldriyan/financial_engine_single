use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use crate::core::errors::EngineError;

/// ============================================================================
/// 💰 Money - මුදල් ව්‍යුහය
/// ============================================================================
/// මෙය පද්ධතියේ ඇති වැදගත්ම දත්ත ව්‍යුහයයි.
/// මූල්‍ය අගයන් ගබඩා කිරීම සඳහා අපි 'float' භාවිතා නොකරමු.
/// ඒ වෙනුවට, අපි කුඩාම ඒකකය (සත - cents) ලෙස 'i64' භාවිතා කරමු.
/// උදාහරණයක් ලෙස: 
/// රු. 10.50 => 1050 (සත)
/// මෙය ගණිතමය දෝෂ (floating point errors) සම්පූර්ණයෙන්ම ඉවත් කරයි.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Money {
    /// අගය සත වලින් (Value in cents)
    pub amount: i64,
}

impl Money {
    /// 🚀 ශුන්‍ය අගයක් සාදන්න (Create zero value)
    pub fn zero() -> Self {
        Money { amount: 0 }
    }

    /// 💵 රුපියල් සහ සත වලින් මුදලක් සාදන්න
    /// (Create from major and minor units)
    /// Ex: Money::new(100, 50) => Rs. 100.50
    pub fn new(rupees: i64, cents: i64) -> Self {
        Money {
            amount: rupees * 100 + cents,
        }
    }

    /// 🔢 සත වලින් කෙලින්ම සාදන්න (Create from cents)
    pub fn from_cents(cents: i64) -> Self {
        Money { amount: cents }
    }

    /// 📈 Float අගයකින් සාදන්න (පරිස්සමෙන් භාවිතා කරන්න)
    /// (Create from float - use with caution)
    pub fn from_float(val: f64) -> Self {
        let cents = (val * 100.0).round() as i64;
        Money { amount: cents }
    }

    /// 🔄 Float එකක් ලෙස ලබාගන්න (දර්ශනය සඳහා පමණි)
    /// (Get as float - for display only)
    pub fn to_float(&self) -> f64 {
        self.amount as f64 / 100.0
    }

    /// ➕ ප්‍රතිශතයක් එකතු කරන්න (Add percentage)
    /// Ex: Rs. 100 + 10% = Rs. 110
    pub fn add_percentage(&self, percentage: f64) -> Self {
        let increase = (self.amount as f64 * (percentage / 100.0)).round() as i64;
        Money {
            amount: self.amount + increase,
        }
    }

    /// ➖ ප්‍රතිශතයක් අඩු කරන්න (Subtract percentage)
    /// Ex: Rs. 100 - 10% = Rs. 90
    pub fn sub_percentage(&self, percentage: f64) -> Self {
        let decrease = (self.amount as f64 * (percentage / 100.0)).round() as i64;
        Money {
            amount: self.amount - decrease,
        }
    }

    /// ➗ කොටස් වලට බෙදන්න (Split into N parts)
    /// ඉතිරිය (remainder) අවසාන කොටසට එකතු වේ.
    pub fn split(&self, parts: i64) -> Result<Vec<Money>, EngineError> {
        if parts <= 0 {
            return Err(EngineError::Calculation{
                code: "INVALID_SPLIT".to_string(),
                message: "කොටස් ගණන 0 ට වැඩි විය යුතුය".to_string()
            });
        }

        let base_amount = self.amount / parts;
        let remainder = self.amount % parts;
        let mut results = Vec::new();

        for i in 0..parts {
            let amount = if i == parts - 1 {
                base_amount + remainder
            } else {
                base_amount
            };
            results.push(Money { amount });
        }

        Ok(results)
    }

    /// ✅ ධන අගයක්ද? (Is positive?)
    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    /// 🛑 ඍණ අගයක්ද? (Is negative?)
    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    /// 🚫 ශුන්‍යද? (Is zero?)
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    /// 🔄 නිරපේක්ෂ අගය (Absolute value)
    pub fn abs(&self) -> Self {
        Money {
            amount: self.amount.abs(),
        }
    }

    /// 📊 ප්‍රතිශතයක් ගණනය කිරීම (Calculate percentage)
    pub fn percentage_of(&self, percentage: f64) -> Self {
        let val = (self.amount as f64 * (percentage / 100.0)).round() as i64;
        Money { amount: val }
    }
}

/// ============================================================================
/// ➕ ගණිතමය ක්‍රියාකාරකම් (Arithmetic Operations)
/// ============================================================================

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Money {
            amount: self.amount + other.amount,
        }
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Money {
            amount: self.amount - other.amount,
        }
    }
}

impl Mul<i64> for Money {
    type Output = Self;

    fn mul(self, scalar: i64) -> Self {
        Money {
            amount: self.amount * scalar,
        }
    }
}

impl Div<i64> for Money {
    type Output = Self;

    fn div(self, scalar: i64) -> Self {
        // Integer division (rounding down)
        Money {
            amount: self.amount / scalar,
        }
    }
}

/// ============================================================================
/// 🔍 සංසන්දනය කිරීම් (Comparisons)
/// ============================================================================

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.amount.cmp(&other.amount))
    }
}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.cmp(&other.amount)
    }
}

/// ============================================================================
/// 📝 දර්ශනය කිරීම (Display)
/// ============================================================================

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let abs_val = self.amount.abs();
        let rupees = abs_val / 100;
        let cents = abs_val % 100;
        let sign = if self.amount < 0 { "-" } else { "" };
        write!(f, "{}Rs.{}.{:02}", sign, rupees, cents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = Money::new(10, 50); // Rs. 10.50
        let b = Money::new(5, 75);  // Rs. 5.75
        let sum = a + b;
        assert_eq!(sum.amount, 1625); // Rs. 16.25
    }

    #[test]
    fn test_split() {
        let total = Money::new(100, 0); // Rs. 100.00
        let parts = total.split(3).unwrap();
        // 33.33 + 33.33 + 33.34 = 100.00
        assert_eq!(parts[0].amount, 3333);
        assert_eq!(parts[1].amount, 3333);
        assert_eq!(parts[2].amount, 3334);
    }
}
