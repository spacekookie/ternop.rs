//! A dead simple ternary operator macro which allows you to write
//! quick one-line returnes for a variety of types.
//!
//! # Examples
//! 
//! To use the ternary operator just invoke the `ternary` macro
//!
//! ```rust
//! fn is_ipv4(val: &str) -> i32 {
//!     ternary!(val == "ipv4", 4, 16)
//! }
//! ```

#[macro_export]
macro_rules! ternary {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition { $_true } else { $_false }
    };
}
