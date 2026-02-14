//! # 💳 Advanced Payment Processor for POS
//! Handles Split Payments, Cheques, Vouchers, and Mix Methods.

use crate::ledger::engine::JournalEntry;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 1. Payment Types supported by POS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    Card {
        last4: String,
        terminal_id: String,
    },
    Cheque {
        number: String,
        bank: String,
        cheque_date: NaiveDate,
    }, // Post-dated support
    Credit {
        customer_account_id: String,
    }, // Pay later (Credit Sale)
    GiftVoucher {
        code: String,
    },
    FidelityPoints {
        points: u32,
        conversion_rate: Decimal,
    },
}

// 2. A Single Payment Part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentComponent {
    pub method: PaymentMethod,
    pub amount: Decimal,
}

// 3. The Complex POS Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosTransactionRequest {
    pub order_id: String,
    pub shop_id: Uuid,
    pub customer_id: Option<Uuid>, // Optional for walking customers
    pub total_amount: Decimal,
    pub payments: Vec<PaymentComponent>, // ✅ List of mix payments
}

pub struct AdvancedPaymentEngine;

impl AdvancedPaymentEngine {
    /// Convert Mixed Payments into Double-Entry Ledger format
    pub fn build_ledger_entries(
        req: PosTransactionRequest,
        revenue_account: Uuid,           // Sales Account
        receivable_account: Uuid,        // Accounts Receivable (for Credit)
        uncleared_cheques_account: Uuid, // For Cheques
        cash_account: Uuid,
        bank_account: Uuid,
    ) -> Result<Vec<JournalEntry>, String> {
        // Returns entries to be posted

        let mut entries = Vec::new();
        let transaction_id = Uuid::new_v4();
        let mut total_paid = Decimal::ZERO;

        for payment in req.payments {
            total_paid += payment.amount;

            // DETERMINE DEBIT ACCOUNT (Where money goes IN)
            let (target_account, description) = match payment.method {
                PaymentMethod::Cash => (cash_account, "Cash Sale".to_string()),
                PaymentMethod::Card { last4, .. } => (bank_account, format!("Card ****{}", last4)),
                // Cheques don't go to Bank immediately! They go to Uncleared/PDC account
                PaymentMethod::Cheque { number, bank, .. } => (
                    uncleared_cheques_account,
                    format!("Cheque {} ({})", number, bank),
                ),
                PaymentMethod::Credit { .. } => (receivable_account, "Credit Sale".to_string()),
                PaymentMethod::GiftVoucher { code } => (
                    // Logic to find Voucher Liability Account would go here
                    // mocking ID for now
                    Uuid::new_v4(),
                    format!("Voucher Redempt: {}", code),
                ),
                _ => (cash_account, "Other".to_string()),
            };

            // DEBIT ENTRY (Asset Up)
            entries.push(JournalEntry {
                id: Uuid::new_v4(),
                transaction_id,
                account_id: target_account,
                debit: payment.amount,
                credit: Decimal::ZERO,
                description,
                created_at: Utc::now(),
            });
        }

        // Validate Totals
        if total_paid != req.total_amount {
            return Err(format!(
                "Payment mismatch! Bill: {}, Paid: {}",
                req.total_amount, total_paid
            ));
        }

        // CREDIT ENTRY (Revenue Up) -> One single entry for Total Sale
        entries.push(JournalEntry {
            id: Uuid::new_v4(),
            transaction_id,
            account_id: revenue_account,
            debit: Decimal::ZERO,
            credit: req.total_amount,
            description: format!("POS Sale Order #{}", req.order_id),
            created_at: Utc::now(),
        });

        Ok(entries)
    }
}
