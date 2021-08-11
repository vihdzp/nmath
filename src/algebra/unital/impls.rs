//! Implements the appropriate identity traits for all primitives.

use super::{LeftUnital, RightUnital};
use crate::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Sub};

/// Implements either a left-sided or a right-sided identity.
macro_rules! impl_sided_id {
    ($type:ty, $op:ident, $sided_identity:ident, $sided_id:ident, $id:expr) => {
        impl $sided_identity<$op> for $type {
            fn $sided_id() -> Self {
                $id
            }
        }
    };
}

/// Implements a left-sided identity.
macro_rules! impl_left_id {
    ($type:ty, $op:ident, $id:expr) => {
        impl_sided_id!($type, $op, LeftUnital, left_id, $id);
    };
}

/// Implements a right-sided identity.
macro_rules! impl_right_id {
    ($type:ty, $op:ident, $id:expr) => {
        impl_sided_id!($type, $op, RightUnital, right_id, $id);
    };
}

/// Implements both a left-sided and a right-sided identity.
macro_rules! impl_both_id {
    ($type:ty, $op:ident, $id:expr) => {
        impl_left_id!($type, $op, $id);
        impl_right_id!($type, $op, $id);
    };
}

/// Implements the additive identity, the subtractive right-identity, the
/// multiplicative identity, and the divisive right-identity for primitive
/// types (excluding floating point numbers).
macro_rules! impl_arith_id {
    ($($type:ty),*) => {
        $(
            impl_both_id!($type, Add, 0 as $type);
            impl_right_id!($type, Sub, 0 as $type);
            impl_both_id!($type, Mul, 1 as $type);
            impl_right_id!($type, Div, 1 as $type);
        )*
    };
}

impl_arith_id!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

/// Implements the perhaps counterintuitive identities for floating point numbers.
macro_rules! impl_farith_id {
    ($($type:ident),*) => {
        $(
            impl_both_id!($type, Add, $type::NAN);
            impl_both_id!($type, Sub, $type::NAN);
            impl_both_id!($type, Mul, $type::NAN);
            impl_both_id!($type, Div, $type::NAN);
        )*
    };
}

impl_farith_id!(f32, f64);

/// Implements the identities for bitwise operations.
macro_rules! impl_bit_id {
    ($($type:ty),*) => {
        $(
            impl_both_id!($type, BitAnd, !(false as $type));
            impl_both_id!($type, BitOr, false as $type);
            impl_both_id!($type, BitXor, false as $type);
        )*
    };
}

impl_bit_id!(bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
