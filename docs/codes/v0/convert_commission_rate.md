```rust
fn convert_commission_rate(cr: f64) -> Result<[u64; 2]> {
    if 1.0 < cr {
        return Err(eg!("commission rate can exceed 100%"));
    }
    if 0.0 > cr {
        return Err(eg!("commission rate must be a positive float number"));
    }
    Ok([(cr * 10000.0) as u64, 10000])
}
```

