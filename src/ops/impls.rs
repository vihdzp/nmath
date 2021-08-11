use super::*;

/// Implements `UnOp` for any type implementing the corresponding Rust primitive
/// trait.
macro_rules! impl_un_op {
    ($name:ident, $fn:ident) => {
        impl<Lhs: ops::$name> UnOp<$name> for Lhs {
            type Output = Lhs::Output;

            fn un_op(self) -> Self::Output {
                self.$fn()
            }
        }
    };
}

impl_un_op!(Neg, neg);
impl_un_op!(Not, not);

/// Implements `BinOp` for any type implementing the corresponding Rust
/// primitive trait.
macro_rules! impl_bin_op {
    ($name:ident, $fn:ident) => {
        impl<Rhs, Lhs: ops::$name<Rhs>> BinOp<$name, Rhs> for Lhs {
            type Output = Lhs::Output;

            fn bin_op(self, rhs: Rhs) -> Self::Output {
                self.$fn(rhs)
            }
        }
    };
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);
impl_bin_op!(Rem, rem);
impl_bin_op!(BitAnd, bitand);
impl_bin_op!(BitOr, bitor);
impl_bin_op!(BitXor, bitxor);
impl_bin_op!(Shl, shl);
impl_bin_op!(Shr, shr);

/// Implements `BinOpAssign` for any type implementing the corresponding Rust
/// primitive traits.
macro_rules! impl_bin_op_assign {
    ($name:ident, $name_assign:ident, $fn:ident) => {
        impl<Rhs, Lhs: ops::$name_assign<Rhs>> BinOpAssign<$name, Rhs> for Lhs {
            fn bin_op_assign(&mut self, rhs: Rhs) {
                self.$fn(rhs)
            }
        }
    };
}

impl_bin_op_assign!(Add, AddAssign, add_assign);
impl_bin_op_assign!(Sub, SubAssign, sub_assign);
impl_bin_op_assign!(Mul, MulAssign, mul_assign);
impl_bin_op_assign!(Div, DivAssign, div_assign);
impl_bin_op_assign!(Rem, RemAssign, rem_assign);
impl_bin_op_assign!(BitAnd, BitAndAssign, bitand_assign);
impl_bin_op_assign!(BitOr, BitOrAssign, bitor_assign);
impl_bin_op_assign!(BitXor, BitXorAssign, bitxor_assign);
impl_bin_op_assign!(Shl, ShlAssign, shl_assign);
impl_bin_op_assign!(Shr, ShrAssign, shr_assign);

/// Implements a backwards operation-assign for a primitive type.
macro_rules! impl_bws_op_assign {
    ($type:ty, $bws:ident, $fn:ident) => {
        impl BinOpAssign<$bws> for $type {
            fn bin_op_assign(&mut self, rhs: Self) {
                use std::ops::*;
                *self = rhs.$fn(*self);
            }
        }

        impl<'a> BinOpAssign<$bws, &'a Self> for $type {
            fn bin_op_assign(&mut self, rhs: &'a Self) {
                use std::ops::*;
                *self = rhs.$fn(*self);
            }
        }
    };
}

/// Implements backwards subtraction assign and backwards division assign for
/// primitive types.
macro_rules! impl_bws_sub_div_assign {
    ($($type:ty),*) => {
        $(
            impl_bws_op_assign!($type, BwsSub, sub);
            impl_bws_op_assign!($type, BwsDiv, div);
        )*
    };
}

impl_bws_sub_div_assign!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
