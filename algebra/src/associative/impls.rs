//! Implements the appropriate semigroup traits for all primitives.

use crate::impl_associativity;
use crate::ops::{Add, Mul};

/// Implements the semigroup trait under addition and multiplication for a
/// primitive.
macro_rules! impl_arith_sg {
    ($($type:ty),*) => {
        $(
            impl_associativity!(Add: $type);
            impl_associativity!(Mul: $type);
        )*
    };
}

impl_arith_sg!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
