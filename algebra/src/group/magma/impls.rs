use super::*;
use crate::ops::*;

/// Implements the [`Magma<Op>`] trait for an operation and type.
macro_rules! impl_magma {
    ($($type:ty : $($op:ty),*);*) => {
        $($(
            impl Magma<$op> for $type {}
        )*)*
    };
}

impl_magma!(
    u8: Add, Mul;
    u16: Add, Mul;
    u32: Add, Mul;
    u64: Add, Mul;
    u128: Add, Mul;
    i8: Add, Sub, Mul;
    i16: Add, Sub, Mul;
    i32: Add, Sub, Mul;
    i64: Add, Sub, Mul;
    i128: Add, Sub, Mul;
    f32: Add, Sub, Mul, Div;
    f64: Add, Sub, Mul, Div
);
