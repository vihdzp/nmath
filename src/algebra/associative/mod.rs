pub mod impls;

use super::{Magma, MaybeRef};
use crate::ops::{BinOp, BinOpMarker};

/// A [`Magma`] is [associative](https://en.wikipedia.org/wiki/Associative_property)
/// whenever `(a op b) op c = a op (b op c)` for all `a, b, c`.
///
/// It is the implementor's responsability to ratify that associativity holds
/// in a given data structure.
pub trait Associative<Op: BinOpMarker>: Magma<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
    /// Tests whether `(a op b) op c = a op (b op c)` for a given set of entries.
    fn test_assoc<T: MaybeRef<Self>, U: MaybeRef<Self>, V: MaybeRef<Self>>(a: T, b: U, c: V) -> bool
    where
        Self: PartialEq,
    {
        Self::op(Self::op(a.borrow(), b.borrow()), c.borrow()) == Self::op(a, Self::op(b, c))
    }
}
