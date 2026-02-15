use crate::core::money::Money;
use crate::subscription::plan::{BillingCycle, Plan};
use chrono::{DateTime, Utc};
use std::ops::{Div, Mul};

/// ============================================================================
/// 🧾 Subscription Billing (බිල්පත් සැකසීම)
/// ============================================================================

pub struct BillingEngine;

impl BillingEngine {
    /// 📉 Proration Calculation (භාවිත කළ දින ගණනට ගෙවීම)
    /// Calculate how much to charge if a user joins in the middle of a cycle.
    pub fn calculate_prorated_amount(
        plan: &Plan,
        start_date: DateTime<Utc>,
        cycle_end_date: DateTime<Utc>,
    ) -> Money {
        let total_days_in_cycle = match plan.cycle {
            BillingCycle::Monthly => 30, // Simplified. Real engine should use calendar days.
            BillingCycle::Quarterly => 90,
            BillingCycle::Yearly => 365,
            BillingCycle::Custom { days } => days,
        };

        let active_duration = cycle_end_date - start_date;
        let active_days = active_duration.num_days().max(0);

        if active_days == 0 {
            return Money::zero();
        }

        if active_days >= total_days_in_cycle {
            return plan.price;
        }

        // Formula: (Price / Total Days) * Active Days
        // Money doesn't support float multiplication directly usually, so we convert.
        // Assuming Money internal is BigInt or similar.
        // Simplified Logic:
        plan.price.mul(active_days).div(total_days_in_cycle)
    }
}
