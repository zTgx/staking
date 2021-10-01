use bytes::Bytes;

/// staking context handler
pub trait Handler {
    /// Type of Native Coin
    type NativeCoin;

    /// Get Native Coin
    fn getNativeCoin() -> Bytes;
}
