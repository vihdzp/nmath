#![allow(clippy::unnecessary_cast, clippy::float_cmp)]

use super::*;

/// Implements `UnOp<op>` for a given operation.
macro_rules! impl_un_op {
    ($($op:ident, |$var:ident| $fn:expr, { $($type:ty),* });*) => {
        $($(
            impl UnOp<$op> for $type {
                type Output = Self;
                type Err = ();

                fn un_op(&self) -> Result<Self, ()> {
                    let f = |$var: Self| $fn;
                    f(*self)
                }
            }
        )*)*
    };
}

impl_un_op!(
    Neg, |x| Ok(-x),
        {i8, i16, i32, i64, i128, f32, f64};
    Rec, |x| (x != 0 as Self).then(|| (1 as Self) / x).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64};
    Not, |x| Ok(!x),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool}
);

/// Implements `UnOpAssign<op>` for a given operation.
macro_rules! impl_un_op_assign {
    ($($op:ident, |$var:ident| $fn:expr, { $($type:ty),* });*) => {
        $($(
            impl UnOpAssign<$op> for $type {
                type Err = ();

                fn un_op_assign(&mut self) -> Result<(), ()> {
                    let f = |$var: &mut Self| $fn;
                    f(self)
                }
            }
        )*)*
    };
}

impl_un_op_assign!(
    Neg, |x| {*x = -*x; Ok(())},
        {i8, i16, i32, i64, i128, f32, f64};
    Rec, |x| (*x != 0 as Self).then(|| *x = (1 as Self) / *x).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64};
    Not, |x| {*x = !*x; Ok(())},
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool}
);
