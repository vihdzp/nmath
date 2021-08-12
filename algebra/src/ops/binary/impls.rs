use super::*;

/// Implements `BinOp<op>` for a given operation.
macro_rules! impl_bin_op {
    ($($op:ident, |$var_x:ident, $var_y:ident| $fn:expr, { $($type:ty),* });*) => {
        $($(
            impl BinOp<$op> for $type {
                type Output = Self;
                type Err = ();

                fn bin_op(&self, rhs: &Self) -> Result<Self, ()> {
                    let f = |$var_x: Self, $var_y: Self| $fn;
                    f(*self, *rhs)
                }
            }
        )*)*
    };
}

impl_bin_op!(
    Add, |x, y| x.checked_add(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Add, |x, y| Ok(x + y),
        {f32, f64};
    Sub, |x, y| x.checked_sub(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Sub, |x, y| Ok(x - y),
        {f32, f64};
    Mul, |x, y| x.checked_mul(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Mul, |x, y| Ok(x * y),
        {f32, f64};
    Div, |x, y| x.checked_div(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Div, |x, y| Ok(x / y),
        {f32, f64};
    Rem, |x, y| x.checked_rem(y).ok_or(()),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    BitAnd, |x, y| Ok(x & y),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool};
    BitOr, |x, y| Ok(x | y),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool};
    BitXor, |x, y| Ok(x ^ y),
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool}
);

// todo: figure out how to handle different types.
// todo: Shl, Shr.

/// Implements [`BinOp`] for any type implementing the corresponding Rust
/// primitive trait for the assignment operation.
macro_rules! impl_bin_op_assign {
    ($($op:ident, |$var_lhs_x:ident, $var_lhs_y:ident| $fn_lhs:expr, |$var_rhs_x:ident, $var_rhs_y:ident| $fn_rhs:expr, { $($type:ty),* });*) => {
        $($(
            impl BinOpAssign<$op> for $type {
                type Err = ();

                fn bin_op_assign_lhs(&mut self, rhs: &Self) -> Result<(), ()> {
                    let f = |$var_lhs_x: &mut Self, $var_lhs_y: Self| $fn_lhs;
                    f(self, *rhs)
                }

                fn bin_op_assign_rhs(&self, rhs: &mut Self) -> Result<(), ()> {
                    let f = |$var_rhs_x: Self, $var_rhs_y: &mut Self| $fn_rhs;
                    f(*self, rhs)
                }
            }
        )*)*
    };
}

impl_bin_op_assign!(
    Add, |x, y| {*x = (*x).checked_add(y).ok_or(())?; Ok(())},
         |x, y| {*y = x.checked_add(*y).ok_or(())?; Ok(())},
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Add, |x, y| {*x += y; Ok(())}, |x, y| {*y += x; Ok(())},
        {f32, f64};
    Sub, |x, y| {*x = (*x).checked_sub(y).ok_or(())?; Ok(())},
         |x, y| {*y = x.checked_sub(*y).ok_or(())?; Ok(())},
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Sub, |x, y| {*x -= y; Ok(())}, |x, y| {*y = x - *y; Ok(())},
        {f32, f64};
    Mul, |x, y| {*x = (*x).checked_mul(y).ok_or(())?; Ok(())},
         |x, y| {*y = x.checked_mul(*y).ok_or(())?; Ok(())},
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Mul, |x, y| {*x *= y; Ok(())}, |x, y| {*y *= x; Ok(())},
        {f32, f64};
    Div, |x, y| {*x = (*x).checked_div(y).ok_or(())?; Ok(())},
         |x, y| {*y = x.checked_div(*y).ok_or(())?; Ok(())},
        {u8, u16, u32, u64, u128, i8, i16, i32, i64, i128};
    Div, |x, y| {*x /= y; Ok(())}, |x, y| {*y = x / *y; Ok(())},
        {f32, f64}
);
