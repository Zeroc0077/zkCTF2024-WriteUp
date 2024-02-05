# Kid Math
[fib.rs](./src/fib.rs)
```rust
meta.create_gate("add", |meta| {
    let s = meta.query_selector(selector);
    let a = meta.query_advice(col_a, Rotation::cur());
    let b = meta.query_advice(col_b, Rotation::cur());
    let c = meta.query_advice(col_c, Rotation::cur());
    vec![s * (a + b - c)]
});
```