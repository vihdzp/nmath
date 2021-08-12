use super::*;
use crate::ops::Mul;

/// Implements either `LeftAbsorbing<Mul>` or `RightAbsorbing<Mul>` for
/// primitive types where 0 is absorbing.
macro_rules! impl_zero_sided_absorb {
    ($type:ty, $side_absorb:ident, $fn:ident) => {
        impl $side_absorb<Mul> for $type {
            fn $fn() -> Self {
                0 as $type
            }
        }
    };
}

/// Implements `Absorbing<Mul>` for primitive types where 0 is absorbing.
macro_rules! impl_zero_absorb {
    ($($type:ty),*) => {
        $(
            impl_zero_sided_absorb!($type, LeftAbsorbing, left_absorb);
            impl_zero_sided_absorb!($type, RightAbsorbing, right_absorb);
        )*
    };
}

impl_zero_absorb!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

// todo: booleans, floating points (NaN is absorbing!)
