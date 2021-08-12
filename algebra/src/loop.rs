use super::{Quasigroup, Unital};
use crate::ops::BinOpMarker;

/// A [loop](https://en.wikipedia.org/wiki/Quasigroup#Loops) is both [`Unital`]
/// and a [`Quasigroup`].
pub trait Loop<Op: BinOpMarker>: Unital<Op> + Quasigroup<Op> {
    // todo: make the inverse a unary operator

    /// Returns the left inverse of a value.
    fn left_inv(&self) -> Self {
        Self::id().right_div(self)
    }

    /// Evaluates the left inverse of a value and assigns it to `self`.
    fn left_inv_assign(&mut self) {
        Self::id().right_div_assign_rhs(self)
    }

    /// Returns the right inverse of a value.
    fn right_inv(&self) -> Self {
        self.left_div(&Self::id())
    }

    /// Evaluates the right inverse of a value and assigns it to `self`.
    fn right_inv_assign(&mut self) {
        self.left_div_assign_lhs(&Self::id())
    }
}

impl<Op: BinOpMarker, T: Unital<Op> + Quasigroup<Op>> Loop<Op> for T {}

/// A [left Bol loop](https://en.wikipedia.org/wiki/Bol_loop) is a [`Loop`] such
/// that `a op (b op (a op c)) = (a op (b op a)) op c` for all `a, b, c`.
///
/// It is up to the implementor to ratify that this condition holds.
pub trait LeftBolLoop<Op: BinOpMarker>: Loop<Op> {
    /// Tests whether three values satisfy the condition for a left Bol loop.
    fn test_left_bol_loop(a: &Self, b: &Self, c: &Self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = a.quasi_op(c);
        b.quasi_op_assign_rhs(&mut lhs);
        a.quasi_op_assign_rhs(&mut lhs);

        let mut rhs = b.quasi_op(a);
        a.quasi_op_assign_rhs(&mut rhs);
        rhs.quasi_op_assign_lhs(c);

        lhs == rhs
    }
}

/// A [right Bol loop](https://en.wikipedia.org/wiki/Bol_loop) is a [`Loop`]
/// such that `((c op a) op b) op a = c op ((a op b) op a)` for all `a, b, c`.
///
/// It is up to the implementor to ratify that this condition holds.
pub trait RightBolLoop<Op: BinOpMarker>: Loop<Op> {
    /// Tests whether three values satisfy the condition for a right Bol loop.
    fn test_right_bol_loop(a: &Self, b: &Self, c: &Self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = c.quasi_op(a);
        lhs.quasi_op_assign_lhs(b);
        lhs.quasi_op_assign_lhs(a);

        let mut rhs = a.quasi_op(b);
        rhs.quasi_op_assign_lhs(a);
        c.quasi_op_assign_rhs(&mut rhs);

        lhs == rhs
    }
}

/// A [Moufang loop](https://en.wikipedia.org/wiki/Moufang_loop) is both a
/// [`LeftBolLoop`] and a [`RightBolLoop`].
pub trait MoufangLoop<Op: BinOpMarker>: LeftBolLoop<Op> + RightBolLoop<Op> {}

impl<Op: BinOpMarker, T: LeftBolLoop<Op> + RightBolLoop<Op>> MoufangLoop<Op> for T {}
