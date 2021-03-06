//! # Staking core data structure
//!
//! including Staking
//!

use std::collections::{BTreeMap, BTreeSet};
use log::info;

pub mod consensus;
pub mod handler;
pub mod storage;
pub mod context;
mod impls;
pub mod commands;
pub mod constants;
#[macro_use]
pub mod macros;
pub mod pre_check;

pub enum Callable {
    Stake,
    Unstake,
}

#[derive(Debug, Default)]
pub struct Staking <C> {
    /// enable staking or not
    pub enable : bool,

    /// current block height
    pub block_height : u64,

    /// validators and powers at current block height
    pub validators : BTreeMap<String, u64>, 

    /// amount of Native is staking
    pub weight : u64,

    /// amount of redeem on the way
    pub redeem : u64,

    /// staking op transactions set at current block height
    pub txs : BTreeSet<String>,

    /// staking reward rate at current block height
    pub reward_rate : [u64; 2],

    /// reward sent history map; 
    /// including <publickey, amount pair>
    // pub rewards_history : BTreeMap<String, u64>,

    // /// reward candidates
    // pub rewards_candidate : BTreeMap<String, u64>,

    /// reward records
    /// including <publickey, amount>
    pub reward_records : BTreeMap<String, u64>,

    /// limit of CoinBase for rewards
    pub balance : u64,

    /// staking context
    pub context : C,
}

impl <C: handler::Handler + context::Context> Staking <C> {
    pub fn new(context: C) -> Self {
        Self::new_with_context(context)
    }

    pub fn new_with_context(context: C) -> Self {
        Staking {
            enable: true,
            block_height: 2,
            validators: BTreeMap::new(),
            weight: 3,
            redeem: 0,
            txs: BTreeSet::new(),
            reward_rate: [2_u64; 2],
            reward_records: BTreeMap::new(),
            balance: 100_u64,
            context: context,
        }
    }

    pub fn pre_stake(&self) -> bool {
        println!("### staking - pre stake");
        self.context.pre_check()
    }

    /// stake
    /// 1. [node] stake N amounts to become validator( or miner )
    /// 2. [staker] delegate M amounts to validator
    pub fn stake(&self, target: &String, amount: u64) -> bool {
        println!("### staking - stake");

        println!("stake to validator : {}", target);

        self.context.do_stake();
    
        true
    }

    /// unstake
    pub fn unstake(&self, amount: u64) -> bool {
        self.context.do_unstake(amount);

        true
    }

    pub fn call(&self, callable: Callable) {
        match callable {
            Callable::Stake => {
                println!("call stake.");
            },
            Callable::Unstake => {
                println!("call unstake.");
            }
        }
    }





    pub fn end_stake(&self) {
        println!("### staking - end stake");
    }
}

/////////////////////////////////////////
pub fn hello() {
    println!("Hello, staking!");
}
