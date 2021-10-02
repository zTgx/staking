extern crate staking_rs;
use log::info;
use staking_rs::{
    context::Context,
    Staking,
};

struct StakingContext {

}

impl StakingContext {
    pub fn new() -> Self {
        StakingContext {

        }   
    }
}

impl Context for StakingContext {
    fn limits(&self) -> u64 {
        8_000_000_u64
    }
}

pub fn main() {
    let context = StakingContext::new();
    let staking = Staking::new(context);
    let res = staking.stake(&"xxx".to_owned(), 8_u64);
    info!("res : {:?}", res);
}
