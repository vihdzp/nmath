pub mod impls;

use super::Magma;
use crate::ops::BinOpMarker;

/// A [`Magma`] is [commutative](https://en.wikipedia.org/wiki/Commutative_property)
/// whenever `a op b = b op a` for all `a, b`.
///
/// It is the implementor's responsability to ratify that commutativity holds
/// in a given data structure.
pub trait Commutative<Op: BinOpMarker>: Magma<Op> {
    /// Applies a binary operation on two values, assigns the result to the
    /// former.
    ///
    /// For clarity, you should prefer this over [`op_assign_lhs`] and
    /// [`op_assign_rhs`] for commutative magmas.
    fn op_assign(&mut self, rhs: &Self) {
        self.op_assign_lhs(rhs);
    }

    /// Tests whether `a op b = b op a` for a given set of entries.
    fn test_comm(&self, rhs: &Self) -> bool
    where
        Self: PartialEq,
    {
        self.op(rhs) == rhs.op(self)
    }
}
