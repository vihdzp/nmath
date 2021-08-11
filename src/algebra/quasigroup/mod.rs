use super::{Magma, MaybeRef};
use crate::ops::{BinOp, BinOpMarker};

pub mod impls;

/// A [quasigroup](https://en.wikipedia.org/wiki/Quasigroup) is a [`Magma`]
/// together with a left division and right division operator.
pub trait Quasigroup<Op: BinOpMarker>: Magma<Op> + Magma<Self::LInv> + Magma<Self::RInv>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>
        + BinOp<Self::LInv, Self, Output = Self>
        + BinOp<Self::RInv, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>
        + BinOp<Self::LInv, &'b Self, Output = Self>
        + BinOp<Self::RInv, &'b Self, Output = Self>,
{
    /// The operator corresponding to left division. It must always be true that
    /// `a Op (b LInv a) = b`.
    type LInv: BinOpMarker;

    /// The operator corresponding to left division. It must always be true that
    /// `(b RInv a) Op a = b`.
    type RInv: BinOpMarker;

    /// Returns the left division of both values.
    fn left_div<T: MaybeRef<Self>, U: MaybeRef<Self>>(lhs: T, rhs: U) -> Self {
        <Self as Magma<Self::LInv>>::op(lhs, rhs)
    }

    /// Tests whether `a Op (b LInv a) = b` for some values.
    fn test_left_div<T: MaybeRef<Self>, U: MaybeRef<Self>>(a: T, b: U) -> bool
    where
        Self: PartialEq,
    {
        let left_div = Self::left_div(b.borrow(), a.borrow());
        &<Self as Magma<Op>>::op(a, left_div) == b.borrow()
    }

    /// Returns the left division of both values.
    fn right_div<T: MaybeRef<Self>, U: MaybeRef<Self>>(lhs: T, rhs: U) -> Self {
        <Self as Magma<Self::RInv>>::op(lhs, rhs)
    }

    /// Tests whether `a Op (b LInv a) = b` for some values.
    fn test_right_div<T: MaybeRef<Self>, U: MaybeRef<Self>>(a: T, b: U) -> bool
    where
        Self: PartialEq,
    {
        &<Self as Magma<Op>>::op(Self::right_div(b.borrow(), a.borrow()), a) == b.borrow()
    }
}
