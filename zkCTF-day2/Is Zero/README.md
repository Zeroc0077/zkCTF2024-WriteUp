# is Zero
[is_zero.rs](./src/is_zero.rs)
```rust
// b = 1 or a * c = 1
meta.create_gate("is_inv", |meta| {
    let s = meta.query_selector(s_is_zero);
    let a = meta.query_advice(advice[0], Rotation::cur());
    let b = meta.query_advice(advice[1], Rotation::cur());
    let c = meta.query_advice(advice[2], Rotation::cur());
    vec![s * (b - Expression::Constant(F::ONE)) * (a * c - Expression::Constant(F::ONE))]
});
// a*b always equals 0
meta.create_gate("mul", |meta| {
    let s = meta.query_selector(s_is_zero);
    let a = meta.query_advice(advice[0], Rotation::cur());
    let b = meta.query_advice(advice[1], Rotation::cur());
    vec![s * a * b]
});
```