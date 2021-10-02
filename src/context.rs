//! # staking context traits
//!
//! staking context supports staking input params

/// staking context
pub trait Context {
    /// Staking amount limits
    fn limits(&self) -> u64;
}
