pub mod impls;

use crate::{
    ops::{BinOp, BinOpMarker},
    BinOpAssign, Bws,
};

/// Mathematically, a [magma](https://en.wikipedia.org/wiki/Magma_(algebra)) is
/// a set together with a closed binary operation. No further restrictions are
/// imposed.
///
/// # Safety
/// In contrast to [`BinOp`], we require that a magma **does not invoke UB**
/// under any "normal" circumstances. The implementor is free to choose whatever
/// a "normal" circumstance means, but they must be aware of the consequences it
/// entails.
pub trait Magma<Op: BinOpMarker>:
    Sized + BinOp<Op, Self, Output = Self> + BinOpAssign<Op, Self>
{
    /// Applies a binary operation on two values.
    ///
    /// On non-copy types, this will often create a new buffer.
    fn op(&self, rhs: &Self) -> Self {
        unsafe { self.bin_op_unchecked(rhs) }
    }

    /// Applies a binary operation on two values, assigns the result to the
    /// former.
    fn op_assign_lhs(&mut self, rhs: &Self) {
        unsafe {
            self.bin_op_assign_lhs_unchecked(rhs);
        }
    }

    /// Applies a binary operation on two values, assigns the result to the
    /// latter.
    fn op_assign_rhs(&self, rhs: &mut Self) {
        unsafe {
            self.bin_op_assign_rhs_unchecked(rhs);
        }
    }
}

impl<Op: BinOpMarker, T: Magma<Op>> Magma<Bws<Op>> for T {}
