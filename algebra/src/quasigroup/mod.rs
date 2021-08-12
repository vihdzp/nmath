use super::Magma;
use crate::ops::BinOpMarker;

pub mod impls;

/// A [left quasigroup](https://en.wikipedia.org/wiki/Quasigroup) is a [`Magma`]
/// together with a left division operator.
pub trait LeftQuasigroup<Op: BinOpMarker>: Magma<Op> + Magma<Self::LInv> {
    /// The operator corresponding to left division. It must always be true that
    /// `a Op (a LInv b) = b`.
    type LInv: BinOpMarker;

    /// An alias for the main operation of a [`LeftQuasigroup`].
    fn left_quasi_op(&self, rhs: &Self) -> Self {
        <Self as Magma<Op>>::op(self, rhs)
    }

    /// An alias for the main left operation-assign of a [`LeftQuasigroup`].
    fn left_quasi_op_assign_lhs(&mut self, rhs: &Self) {
        <Self as Magma<Op>>::op_assign_lhs(self, rhs)
    }

    /// An alias for the main right operation-assign of a [`LeftQuasigroup`].
    fn left_quasi_op_assign_rhs(&self, rhs: &mut Self) {
        <Self as Magma<Op>>::op_assign_rhs(self, rhs)
    }

    /// Returns the left division of both values.
    fn left_div(&self, rhs: &Self) -> Self {
        <Self as Magma<Self::LInv>>::op(self, rhs)
    }

    /// Evaluates the left division of `self` and `rhs` and assigns it to `self`.
    fn left_div_assign_lhs(&mut self, rhs: &Self) {
        <Self as Magma<Self::LInv>>::op_assign_lhs(self, rhs)
    }

    /// Evaluates the left division of `self` and `rhs` and assigns it to `rhs`.
    fn left_div_assign_rhs(&self, rhs: &mut Self) {
        <Self as Magma<Self::LInv>>::op_assign_rhs(self, rhs)
    }

    /// Tests whether `a Op (a LInv b) = b` for some values.
    fn test_left_div(&self, rhs: &Self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = self.left_div(rhs);
        self.left_quasi_op_assign_rhs(&mut lhs);
        &lhs == rhs
    }
}

/// A [right quasigroup](https://en.wikipedia.org/wiki/Quasigroup) is a
/// [`Magma`] together with a right division operator.
pub trait RightQuasigroup<Op: BinOpMarker>: Magma<Op> + Magma<Self::RInv> {
    /// The operator corresponding to right division. It must always be true
    /// that `(b RInv a) Op a = b`.
    type RInv: BinOpMarker;

    /// An alias for the main operation of a [`RightQuasigroup`].
    fn right_quasi_op(&self, rhs: &Self) -> Self {
        <Self as Magma<Op>>::op(self, rhs)
    }

    /// An alias for the main left operation-assign of a [`RightQuasigroup`].
    fn right_quasi_op_assign_lhs(&mut self, rhs: &Self) {
        <Self as Magma<Op>>::op_assign_lhs(self, rhs)
    }

    /// An alias for the main right operation-assign of a [`RightQuasigroup`].
    fn right_quasi_op_assign_rhs(&self, rhs: &mut Self) {
        <Self as Magma<Op>>::op_assign_rhs(self, rhs)
    }

    /// Returns the left division of both values.
    fn right_div(&self, rhs: &Self) -> Self {
        <Self as Magma<Self::RInv>>::op(self, rhs)
    }

    /// Evaluates the right division of `self` and `rhs` and assigns it to `self`.
    fn right_div_assign_lhs(&mut self, rhs: &Self) {
        <Self as Magma<Self::RInv>>::op_assign_lhs(self, rhs)
    }

    /// Evaluates the right division of `self` and `rhs` and assigns it to `rhs`.
    fn right_div_assign_rhs(&self, rhs: &mut Self) {
        <Self as Magma<Self::RInv>>::op_assign_rhs(self, rhs)
    }

    /// Tests whether `(b RInv a) Op a = b` for some values.
    fn test_right_div(&self, rhs: &Self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = rhs.right_div(self);
        lhs.right_quasi_op_assign_lhs(self);
        &lhs == rhs
    }
}

/// A [quasigroup](https://en.wikipedia.org/wiki/Quasigroup) is a [`Magma`]
/// together with a left division and right division operator.
pub trait Quasigroup<Op: BinOpMarker>: LeftQuasigroup<Op> + RightQuasigroup<Op> {
    /// An alias for the main operation of a [`Quasigroup`].
    fn quasi_op(&self, rhs: &Self) -> Self {
        self.left_quasi_op(rhs)
    }

    /// An alias for the main left operation-assign of a [`Quasigroup`].
    fn quasi_op_assign_lhs(&mut self, rhs: &Self) {
        self.left_quasi_op_assign_lhs(rhs)
    }

    /// An alias for the main right operation-assign of a [`Quasigroup`].
    fn quasi_op_assign_rhs(&self, rhs: &mut Self) {
        self.left_quasi_op_assign_rhs(rhs)
    }
}

impl<Op: BinOpMarker, T: LeftQuasigroup<Op> + RightQuasigroup<Op>> Quasigroup<Op> for T {}
