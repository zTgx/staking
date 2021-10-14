pub fn check_delegation_amount(am: Amount, is_append: bool) -> Result<()> {
    let lowb = alt!(
        is_append,
        MIN_DELEGATION_AMOUNT,
        STAKING_VALIDATOR_MIN_POWER
    );
    if (lowb..=MAX_DELEGATION_AMOUNT).contains(&am) {
        Ok(())
    } else {
        let msg = format!(
            "Invalid delegation amount: {} (min: {}, max: {})",
            am, lowb, MAX_DELEGATION_AMOUNT
        );
        Err(eg!(msg))
    }
}