use ruc::*;
use crate::constants::{AVAILABEL_BLOCK_HEIGHT_INTERVAL, STAKING_COMMISSION_RATE_DENOMINATOR};

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

pub fn format_commission_rate(commission_rate: &str) -> [u64; 2] {
    commission_rate
        .parse::<f64>()
        .c(d!("commission rate must be a float number"))
        .and_then(|cr| 
            if 1.0 < cr {
                return Err(eg!("commission rate can exceed 100%"));
            }
            if 0.0 > cr {
                return Err(eg!("commission rate must be a positive float number"));
            }
            Ok([(cr * STAKING_COMMISSION_RATE_DENOMINATOR as f64) as u64, STAKING_COMMISSION_RATE_DENOMINATOR])
        )?;
}

pub fn check_block_height_interval() -> Result<()> {
    let network_height = get_block_height(get_serv_addr().unwrap());
    let local_height = get_local_block_height();
    if (network_height == 0 || local_height == 0)
        || diff!(network_height, local_height) > AVAILABEL_BLOCK_HEIGHT_INTERVAL
    {
        println!(
            "The difference in block height of your node and the remote network is too big: \n remote / local: {} / {}",
            network_height, local_height
        );
        if !force {
            println!("Append option --force to ignore this warning.");
            return Ok(());
        }
        println!("Continue to stake now...");
    }

    Ok(())
}