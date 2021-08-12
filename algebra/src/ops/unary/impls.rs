use super::*;

/// Implements `UnOp<op>` for a given operation.
macro_rules! impl_un_op {
    ($($op:ident, $fn:expr, { $($type:ty),* });*) => {
        $($(
            impl UnOp<$op> for $type {
                type Output = Self;
                type Err = ();

                fn un_op(&self) -> Result<Self, ()> {
                    ($fn)(*self)
                }
            }
        )*)*
    };
}

impl_un_op!(
    Neg, |x: Self| Ok(-x),
        {i8, i16, i32, i64, i128, f32, f64};
    Rec, |x: Self| (x != 0 as Self).then(|| (1 as Self) / x).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64};
    Not, |x: Self| Ok(!x),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool}
);

/// Implements `UnOpAssign<op>` for a given operation.
macro_rules! impl_un_op_assign {
    ($($op:ident, $fn:expr, { $($type:ty),* });*) => {
        $($(
            impl UnOpAssign<$op> for $type {
                type Err = ();

                fn un_op_assign(&mut self) -> Result<(), ()> {
                    ($fn)(self)
                }
            }
        )*)*
    };
}

impl_un_op_assign!(
    Neg, |x: &mut Self| Ok(*x = -*x),
        {i8, i16, i32, i64, i128, f32, f64};
    Rec, |x: &mut Self| (*x != 0 as Self).then(|| *x = (1 as Self) / *x ).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64};
    Not, |x: &mut Self| Ok(*x = !*x),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool}
);
