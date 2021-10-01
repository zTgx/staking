//! # Staking core data structure
//!
//! including Staking
//!

#[derive(Debug)]
pub struct Staking {
    /// enable staking or not
    pub enable : bool,

    /// current block height
    pub block_height : u64,

    /// validators at current block height
    pub validators : Vec<String>, 

    /// amount of Native is staking
    pub weight : u64,

    /// amount of redeem en the way
    pub redeem : u64,

    /// staking op transactions at current block height
    pub txs : Vec<String>,
}

pub fn hello() {
    println!("Hello, staking!");
}
