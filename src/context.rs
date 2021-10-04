//! # staking context traits
//!
//! staking context supports staking input params

/// staking context
pub trait Context {
    /// Staking amount threshold
    fn threshold(&self) -> u64;

    /// commission rate
    fn commission_rate(&self) -> [u64; 2];

    /// pre check
    fn pre_check(&self) -> bool;

    /// stake type : public or private
    /// aka: stake yourself or stake to validator
    fn is_public_stake_pool(&self) -> bool;
}
