use bytes::Bytes;

/// staking context handler
pub trait Handler {
    /// Staking amount limits
    // fn limits(&self) -> u64;

    /// doStake
    fn do_stake(&self);

    /// doUnstake
    fn do_unstake(&self, amount: u64);
}
