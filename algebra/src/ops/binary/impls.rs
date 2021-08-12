use super::*;

/// Implements `BinOp<op>` for a given operation.
macro_rules! impl_bin_op {
    ($($op:ident, $fn:expr, { $($type:ty),* });*) => {
        $($(
            impl BinOp<$op> for $type {
                type Output = Self;
                type Err = ();

                fn bin_op(&self, rhs: &Self) -> Result<Self, ()> {
                    ($fn)(*self, *rhs)
                }
            }
        )*)*
    };
}

impl_bin_op!(
    Add, |x: Self, y: Self| x.checked_add(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Add, |x: Self, y: Self| Ok(x + y),
        {f32, f64};
    Sub, |x: Self, y: Self| x.checked_sub(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Sub, |x: Self, y: Self| Ok(x - y),
        {f32, f64};
    Mul, |x: Self, y: Self| x.checked_mul(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Mul, |x: Self, y: Self| Ok(x * y),
        {f32, f64};
    Div, |x: Self, y: Self| x.checked_div(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Div, |x: Self, y: Self| Ok(x / y),
        {f32, f64};
    Rem, |x: Self, y: Self| x.checked_rem(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    BitAnd, |x: Self, y: Self| Ok(x & y),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool};
    BitOr, |x: Self, y: Self| Ok(x | y),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool};
    BitXor, |x: Self, y: Self| Ok(x ^ y),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool}
);

// todo: figure out how to handle different types.
// todo: Shl, Shr.

/// Implements [`BinOp`] for any type implementing the corresponding Rust
/// primitive trait for the assignment operation.
macro_rules! impl_bin_op_assign {
    ($($op:ident, $fn_lhs:expr, $fn_rhs:expr, { $($type:ty),* });*) => {
        $($(
            impl BinOpAssign<$op> for $type {
                type Err = ();

                fn bin_op_assign_lhs(&mut self, rhs: &Self) -> Result<(), ()> {
                    ($fn_lhs)(self, *rhs)
                }

                fn bin_op_assign_rhs(&self, rhs: &mut Self) -> Result<(), ()> {
                    ($fn_rhs)(*self, rhs)
                }
            }
        )*)*
    };
}

impl_bin_op_assign!(
    Add, |x: &mut Self, y: Self| Ok(*x = (*x).checked_add(y).ok_or(())?),
         |x: Self, y: &mut Self| Ok(*y = x.checked_add(*y).ok_or(())?),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Add, |x: &mut Self, y: Self| Ok(*x += y), |x: Self, y: &mut Self| Ok(*y += x),
        {f32, f64};
    Sub, |x: &mut Self, y: Self| Ok(*x = (*x).checked_sub(y).ok_or(())?),
         |x: Self, y: &mut Self| Ok(*y = x.checked_sub(*y).ok_or(())?),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Sub, |x: &mut Self, y: Self| Ok(*x -= y), |x: Self, y: &mut Self| Ok(*y = x - *y),
        {f32, f64};
    Mul, |x: &mut Self, y: Self| Ok(*x = (*x).checked_mul(y).ok_or(())?),
         |x: Self, y: &mut Self| Ok(*y = x.checked_mul(*y).ok_or(())?),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Mul, |x: &mut Self, y: Self| Ok(*x *= y), |x: Self, y: &mut Self| Ok(*y *= x),
        {f32, f64};
    Div, |x: &mut Self, y: Self| Ok(*x = (*x).checked_div(y).ok_or(())?),
         |x: Self, y: &mut Self| Ok(*y = x.checked_div(*y).ok_or(())?),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Div, |x: &mut Self, y: Self| Ok(*x /= y), |x: Self, y: &mut Self| Ok(*y = x / *y),
        {f32, f64}
);
