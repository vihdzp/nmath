//! Contains the traits for left-unital, right-unital, and unital magmas.

pub mod impls;

use super::{Magma, MaybeRef};
use crate::ops::{BinOp, BinOpMarker};

/// A trait that specifies that a [`Magma`] has a unique
/// [left identity](https://en.wikipedia.org/wiki/Identity_element#Definitions).
pub trait LeftUnital<Op: BinOpMarker>: Magma<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
    /// Returns the unique left identity.
    fn left_id() -> Self;

    /// Tests whether the left identity times a given value equals the value.
    fn test_left_id<T: MaybeRef<Self>>(a: T) -> bool
    where
        Self: PartialEq,
    {
        &Self::op(Self::left_id(), a.borrow()) == a.borrow()
    }
}

/// A trait that specifies that a [`Magma`] has a unique
/// [right identity](https://en.wikipedia.org/wiki/Identity_element#Definitions).
pub trait RightUnital<Op: BinOpMarker>: Magma<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
    /// Returns the unique right identity.
    fn right_id() -> Self;

    /// Tests whether the given value times the right identity equals the value.
    fn test_left_id<T: MaybeRef<Self>>(a: T) -> bool
    where
        Self: PartialEq,
    {
        &Self::op(a.borrow(), Self::right_id()) == a.borrow()
    }
}

/// A [`Magma`] is unital whenever it has a (unique) two-sided identity element.
pub trait Unital<Op: BinOpMarker>: LeftUnital<Op> + RightUnital<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
    /// Returns the unique identity.
    fn id() -> Self {
        Self::left_id()
    }

    /// Checks whether the left and right identities are equal.
    fn test_id() -> bool
    where
        Self: PartialEq,
    {
        Self::left_id() == Self::right_id()
    }
}

impl<Op: BinOpMarker, T: LeftUnital<Op> + RightUnital<Op>> Unital<Op> for T
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
}
