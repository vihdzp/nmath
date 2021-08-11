pub mod impls;

use super::{Magma, MaybeRef};
use crate::ops::{BinOp, BinOpMarker};

/// A [`Magma`] is [commutative](https://en.wikipedia.org/wiki/Commutative_property)
/// whenever `a op b = b op a` for all `a, b`.
///
/// It is the implementor's responsability to ratify that commutativity holds
/// in a given data structure.
pub trait Commutative<Op: BinOpMarker>: Magma<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
    /// Tests whether `a op b = b op a` for a given set of entries.
    fn test_assoc<T: MaybeRef<Self>, U: MaybeRef<Self>>(a: T, b: U) -> bool
    where
        Self: PartialEq,
    {
        Self::op(a.borrow(), b.borrow()) == Self::op(a, b)
    }
}
