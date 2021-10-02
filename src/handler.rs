use bytes::Bytes;

/// staking context handler
pub trait Handler {
    /// Staking amount limits
    fn limits(&self) -> u64;
}
