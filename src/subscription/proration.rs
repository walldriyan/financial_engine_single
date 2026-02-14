use crate::core::errors::{EngineError, EngineResult};
use crate::core::money::Money;
use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};

/// ============================================================================
/// 📅 Proration Engine (අනුපාත ගණනය කිරීමේ එන්ජිම)
/// ============================================================================
/// Subscription billing proration for:
/// - Mid-cycle upgrades/downgrades
/// - Pro-rated refunds
/// - Usage-based billing
/// - Trial period calculations

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProrationRequest {
    pub subscription_id: String,
    pub old_plan_amount: Money,
    pub new_plan_amount: Money,
    pub billing_cycle_start: DateTime<Utc>,
    pub billing_cycle_end: DateTime<Utc>,
    pub change_date: DateTime<Utc>,
    pub proration_method: ProrationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProrationMethod {
    /// Days used / Total days
    DayBased,
    /// Seconds used / Total seconds (most accurate)
    SecondBased,
    /// No proration - charge full amount immediately
    None,
    /// Prorate and apply as credit to next invoice
    CreditNext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProrationResult {
    pub subscription_id: String,
    pub credit_amount: Money, // Credit from old plan (unused portion)
    pub charge_amount: Money, // Charge for new plan (remaining portion)
    pub net_amount: Money,    // Net amount to charge (can be negative = credit)
    pub days_remaining: i64,
    pub days_total: i64,
    pub proration_factor: f64, // 0.0 to 1.0
    pub effective_date: DateTime<Utc>,
    pub next_billing_date: DateTime<Utc>,
}

/// 🧮 Proration Calculator (අනුපාත ගණක)
pub struct ProrationEngine;

impl ProrationEngine {
    /// 💰 Calculate proration for plan change
    pub fn calculate(request: &ProrationRequest) -> EngineResult<ProrationResult> {
        // Validate dates
        if request.change_date < request.billing_cycle_start {
            return Err(EngineError::Validation {
                message: "Change date cannot be before billing cycle start".to_string(),
            });
        }
        if request.change_date > request.billing_cycle_end {
            return Err(EngineError::Validation {
                message: "Change date cannot be after billing cycle end".to_string(),
            });
        }

        // Calculate time factors
        let total_seconds = (request.billing_cycle_end - request.billing_cycle_start).num_seconds();
        let _used_seconds = (request.change_date - request.billing_cycle_start).num_seconds();
        let remaining_seconds = (request.billing_cycle_end - request.change_date).num_seconds();

        let total_days = (request.billing_cycle_end - request.billing_cycle_start).num_days();
        let remaining_days = (request.billing_cycle_end - request.change_date).num_days();

        if total_seconds <= 0 {
            return Err(EngineError::Validation {
                message: "Invalid billing cycle duration".to_string(),
            });
        }

        let proration_factor = match request.proration_method {
            ProrationMethod::SecondBased => remaining_seconds as f64 / total_seconds as f64,
            ProrationMethod::DayBased => remaining_days as f64 / total_days as f64,
            ProrationMethod::None => 1.0,
            ProrationMethod::CreditNext => remaining_seconds as f64 / total_seconds as f64,
        };

        // Calculate credit from old plan (unused portion)
        let credit_amount =
            Self::calculate_prorated_amount(&request.old_plan_amount, proration_factor);

        // Calculate charge for new plan (remaining portion)
        let charge_amount =
            Self::calculate_prorated_amount(&request.new_plan_amount, proration_factor);

        // Net amount = New charges - Old credits
        let net_amount = charge_amount - credit_amount;

        Ok(ProrationResult {
            subscription_id: request.subscription_id.clone(),
            credit_amount,
            charge_amount,
            net_amount,
            days_remaining: remaining_days,
            days_total: total_days,
            proration_factor,
            effective_date: request.change_date,
            next_billing_date: request.billing_cycle_end,
        })
    }

    /// Calculate prorated amount
    fn calculate_prorated_amount(amount: &Money, factor: f64) -> Money {
        let prorated = (amount.amount as f64 * factor).round() as i64;
        Money::from_cents(prorated)
    }

    /// 📊 Calculate trial period remaining charges
    pub fn trial_remaining(
        trial_start: DateTime<Utc>,
        trial_end: DateTime<Utc>,
        full_plan_amount: Money,
        conversion_date: DateTime<Utc>,
    ) -> EngineResult<Money> {
        if conversion_date < trial_start {
            return Err(EngineError::Validation {
                message: "Conversion date cannot be before trial start".to_string(),
            });
        }

        if conversion_date >= trial_end {
            // Trial ended, charge full amount
            return Ok(full_plan_amount);
        }

        // Calculate remaining trial days
        let total_trial_days = (trial_end - trial_start).num_days();
        let _remaining_trial_days = (trial_end - conversion_date).num_days();

        if total_trial_days <= 0 {
            return Ok(full_plan_amount);
        }

        // No charge for remaining trial period (they already paid nothing)
        // But if they convert early, we might give credit
        // This returns what they should pay for a full cycle starting now
        Ok(full_plan_amount)
    }

    /// 📈 Calculate usage-based billing
    pub fn usage_based(
        base_amount: Money,
        included_units: f64,
        actual_units: f64,
        overage_rate: Money, // Per unit overage cost
    ) -> EngineResult<UsageBillingResult> {
        if actual_units <= included_units {
            return Ok(UsageBillingResult {
                base_charge: base_amount,
                overage_units: 0.0,
                overage_charge: Money::zero(),
                total_charge: base_amount,
                units_remaining: included_units - actual_units,
            });
        }

        let overage_units = actual_units - included_units;
        let overage_charge = overage_rate * (overage_units.ceil() as i64);
        let total_charge = base_amount + overage_charge;

        Ok(UsageBillingResult {
            base_charge: base_amount,
            overage_units,
            overage_charge,
            total_charge,
            units_remaining: 0.0,
        })
    }

    /// 🔄 Calculate refund for cancellation
    pub fn cancellation_refund(
        current_plan_amount: Money,
        billing_cycle_start: DateTime<Utc>,
        billing_cycle_end: DateTime<Utc>,
        cancellation_date: DateTime<Utc>,
        refund_policy: RefundPolicy,
    ) -> EngineResult<CancellationResult> {
        let total_days = (billing_cycle_end - billing_cycle_start).num_days();
        let used_days = (cancellation_date - billing_cycle_start).num_days();
        let remaining_days = (billing_cycle_end - cancellation_date).num_days();

        if total_days <= 0 {
            return Err(EngineError::Validation {
                message: "Invalid billing cycle".to_string(),
            });
        }

        let refund_amount = match refund_policy {
            RefundPolicy::FullRefund => current_plan_amount,
            RefundPolicy::NoRefund => Money::zero(),
            RefundPolicy::Prorated => {
                let factor = remaining_days as f64 / total_days as f64;
                Self::calculate_prorated_amount(&current_plan_amount, factor)
            }
            RefundPolicy::GracePeriod { days } => {
                if used_days <= days {
                    current_plan_amount
                } else {
                    Money::zero()
                }
            }
        };

        Ok(CancellationResult {
            refund_amount,
            effective_end_date: cancellation_date,
            days_unused: remaining_days,
            days_used: used_days,
            refund_policy,
        })
    }
}

/// 📊 Usage Billing Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageBillingResult {
    pub base_charge: Money,
    pub overage_units: f64,
    pub overage_charge: Money,
    pub total_charge: Money,
    pub units_remaining: f64,
}

/// 🚫 Cancellation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancellationResult {
    pub refund_amount: Money,
    pub effective_end_date: DateTime<Utc>,
    pub days_unused: i64,
    pub days_used: i64,
    pub refund_policy: RefundPolicy,
}

/// 📋 Refund Policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefundPolicy {
    FullRefund,
    NoRefund,
    Prorated,
    GracePeriod { days: i64 },
}

/// 📅 Billing Cycle Calculator
pub struct BillingCycleCalculator;

impl BillingCycleCalculator {
    /// Calculate next billing date
    pub fn next_billing_date(current: DateTime<Utc>, cycle: BillingCycle) -> DateTime<Utc> {
        match cycle {
            BillingCycle::Daily => current + Duration::days(1),
            BillingCycle::Weekly => current + Duration::weeks(1),
            BillingCycle::Monthly => {
                let month = current.month();
                let year = current.year();
                let (new_year, new_month) = if month == 12 {
                    (year + 1, 1)
                } else {
                    (year, month + 1)
                };
                current
                    .with_year(new_year)
                    .unwrap()
                    .with_month(new_month)
                    .unwrap()
            }
            BillingCycle::Quarterly => {
                let month = current.month();
                let year = current.year();
                let new_month = month + 3;
                if new_month > 12 {
                    current
                        .with_year(year + 1)
                        .unwrap()
                        .with_month(new_month - 12)
                        .unwrap()
                } else {
                    current.with_month(new_month).unwrap()
                }
            }
            BillingCycle::Yearly => current.with_year(current.year() + 1).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingCycle {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proration_upgrade() {
        let request = ProrationRequest {
            subscription_id: "SUB001".to_string(),
            old_plan_amount: Money::new(100, 0), // Rs. 100/month
            new_plan_amount: Money::new(200, 0), // Rs. 200/month
            billing_cycle_start: Utc::now() - Duration::days(15),
            billing_cycle_end: Utc::now() + Duration::days(15),
            change_date: Utc::now(),
            proration_method: ProrationMethod::DayBased,
        };

        let result = ProrationEngine::calculate(&request).unwrap();

        // Mid-cycle upgrade: 15 days remaining out of 30
        // Credit: 100 * 0.5 = 50
        // Charge: 200 * 0.5 = 100
        // Net: 100 - 50 = 50
        assert!(result.net_amount.is_positive());
        assert_eq!(result.proration_factor, 0.5);
    }

    #[test]
    fn test_usage_billing() {
        let result = ProrationEngine::usage_based(
            Money::new(50, 0), // Base: Rs. 50
            100.0,             // Included: 100 units
            150.0,             // Actual: 150 units
            Money::new(1, 0),  // Overage: Rs. 1/unit
        )
        .unwrap();

        // Overage: 50 units * Rs. 1 = Rs. 50
        // Total: Rs. 50 + Rs. 50 = Rs. 100
        assert_eq!(result.overage_units, 50.0);
        assert_eq!(result.overage_charge.amount, 5000);
        assert_eq!(result.total_charge.amount, 10000);
    }

    #[test]
    fn test_cancellation_prorated() {
        let result = ProrationEngine::cancellation_refund(
            Money::new(100, 0),
            Utc::now() - Duration::days(10),
            Utc::now() + Duration::days(20),
            Utc::now(),
            RefundPolicy::Prorated,
        )
        .unwrap();

        // Used 10 days, 20 remaining out of 30
        // Refund: 100 * (20/30) = ~66.67
        assert!(result.refund_amount.is_positive());
        assert_eq!(result.days_used, 10);
        assert_eq!(result.days_unused, 20);
    }
}
