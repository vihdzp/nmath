use super::{LeftQuasigroup, RightQuasigroup};
use crate::ops::{Add, Bws, Sub};

/// Declares a primitive type as a quasigroup under a given commmutative
/// operation, its both-sided "inverse", and the backwards of that inverse.
macro_rules! impl_quasi {
    ($type:ty, $op:ty, $inv:ty) => {
        impl LeftQuasigroup<$op> for $type {
            type LInv = Bws<$inv>;
        }

        impl RightQuasigroup<$op> for $type {
            type RInv = $inv;
        }

        impl LeftQuasigroup<$inv> for $type {
            type LInv = $inv;
        }

        impl RightQuasigroup<$inv> for $type {
            type RInv = $op;
        }

        impl LeftQuasigroup<Bws<$inv>> for $type {
            type LInv = $op;
        }

        impl RightQuasigroup<Bws<$inv>> for $type {
            type RInv = Bws<$inv>;
        }
    };
}

/// Declares a primitive type as a quasigroup under addition, subtraction, and
/// backwards subtraction.
macro_rules! impl_quasi_add {
    ($($type:ty),*) => {
        $(
            impl_quasi!($type, Add, Sub);
        )*
    };
}

impl_quasi_add!(i8, i16, i32, i64, i128);
