```rust
/// Retrieve current block height of the specified tendermint node address
pub fn get_block_height(addr: &str) -> u64 {
    get_network_status(addr)
        .map(|ts| ts.sync_info.latest_block_height.parse::<u64>().unwrap())
        .unwrap_or(0)
}
```

