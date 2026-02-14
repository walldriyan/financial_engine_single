#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn test_banking_precision_reversal() {
        // 1. Initial State (Balance = 0)
        let mut balance = dec!(0.00);
        println!("Initial Balance: {}", balance);

        // 2. Add a complex amount (Transaction)
        // උදාහරණයක්: රු. 3333.3333333333333333
        let amount = dec!(3333.3333333333333333); 
        balance += amount;
        println!("After Credit:  {}", balance);

        // 3. Multi-step Operations (Tax, Fee separation) to introduce noise
        let tax = amount * dec!(0.1); // 10%
        let net = amount - tax;       // 90%
        // Re-combine (Simulation of split entry)
        let reconstructed = net + tax;

        assert_eq!(amount, reconstructed, "Split math must match exactly");

        // 4. Reverse the Transaction (Debit)
        // හරියටම අර ගාණම අඩු කරනවා.
        balance -= amount;
        println!("After Reversal: {}", balance);

        // 5. Final Check (The Truth)
        // සාමාන්‍ය Float (f64) වල මෙය 0.0000000000001 වෙනවා.
        // නමුත් Decimal වල මෙය හරියටම ZERO විය යුතුයි.
        assert_eq!(balance, dec!(0.00), "Balance must come back to ZERO exactly without debris");
        assert_eq!(balance.is_zero(), true);
        
        println!("✅ Precision Check Passed: No decimal dust left!");
    }
}
