extern crate staking_rs;
use staking_rs::{Cli, Amount};

pub fn main() {
    let amount  = 1000_u64;
    let commission_rate = 0.2;
    let memo = "memo";
    
    Cli::stake(amount, commission_rate, memo)
}