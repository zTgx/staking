```rust
pub fn get_keypair() -> Result<XfrKeyPair> {
    if let Some(m_path) = MNEMONIC.as_ref() {
        fs::read_to_string(m_path)
            .c(d!("can not read mnemonic from 'owner-mnemonic-path'"))
            .and_then(|m| {
                wallet::restore_keypair_from_mnemonic_default(m.trim())
                    .c(d!("invalid 'owner-mnemonic'"))
            })
    } else {
        Err(eg!("'owner-mnemonic-path' has not been set"))
    }
}
```

