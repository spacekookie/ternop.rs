# Ternary Operator

Rust doesn't support `return (condition) ? if_true : if_false;`. This crate exports a macro that implements this feature.

```rust
fn is_ipv4(val: &str) -> i32 {
    ternary!(val == "ipv4", 4, 16)
}
```

If you just want to copy the small macro, here you go 😅

```
#[macro_export]
macro_rules! ternary {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition { $_true } else { $_false }
    };
}
```