//! Implements the appropriate semigroup traits for all primitives.

use super::Commutative;
use crate::ops::{Add, Mul};

/// Implements the semigroup trait under addition and multiplication for a
/// primitive.
macro_rules! impl_arith_sg {
    ($($type:ty),*) => {
        $(
            impl Commutative<Add> for $type {}
            impl Commutative<Mul> for $type {}
        )*
    };
}

impl_arith_sg!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
