extern crate staking_rs;
use log::info;
use staking_rs::{
    context::Context,
    Staking,
    handler::Handler,
    Callable,
};

#[derive(Debug)]
struct StakingContext {}
impl StakingContext {
    pub fn new() -> Self {
        StakingContext {}   
    }
}

impl Context for StakingContext {
    fn threshold(&self) -> u64 {
        8_000_000_u64
    }

    fn commission_rate(&self) -> [u64; 2] {
        [1_u64, 1_000_000_u64]
    }

    fn pre_check(&self) -> bool {
        // diff(network_height, local_height) < 3
        true
    }

    fn is_public_stake_pool(&self) -> bool {
        true
    }
}

impl Handler for StakingContext {
    fn do_stake(&self) {
        println!("### do stake : {:?}", self);
    }

    fn do_unstake(&self, amount: u64) {
        println!("### do un-stake amount : {}", amount);
    }
}

pub fn main() {
    let context = StakingContext::new();
    let staking = Staking::new(context);

    let _ = staking.pre_stake();
    
    let res = staking.stake(&"xxx".to_owned(), 8_u64);
    info!("res : {:?}", res);
    let res = staking.unstake(3_u64);
    println!("unstake res : {}", res);

    let c = Callable::Stake;
    staking.call(c);
}
