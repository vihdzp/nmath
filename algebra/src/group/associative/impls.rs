//! Implements the appropriate semigroup traits for all primitives.

use crate::ops::{Add, Mul};

/// Implements [`PowAssociative`] for a type.
macro_rules! impl_associativity {
    ($op:ty: $type:ty) => {
        impl crate::PowAssociative<$op> for $type {}
    };
}

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
