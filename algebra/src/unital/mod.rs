//! Contains the traits for left-unital, right-unital, and unital magmas.

use super::Magma;
use crate::ops::BinOpMarker;

/// A trait that specifies that a [`Magma`] has a unique
/// [left identity](https://en.wikipedia.org/wiki/Identity_element#Definitions).
pub trait LeftUnital<Op: BinOpMarker>: Magma<Op> {
    /// Returns the unique left identity.
    fn left_id() -> Self;

    /// Tests whether the left identity times a given value equals the value.
    fn test_left_id(&self) -> bool
    where
        Self: PartialEq,
    {
        &Self::left_id().op(self) == self
    }
}

/// A trait that specifies that a [`Magma`] has a unique
/// [right identity](https://en.wikipedia.org/wiki/Identity_element#Definitions).
pub trait RightUnital<Op: BinOpMarker>: Magma<Op> {
    /// Returns the unique right identity.
    fn right_id() -> Self;

    /// Tests whether the given value times the right identity equals the value.
    fn test_left_id(&self) -> bool
    where
        Self: PartialEq,
    {
        &self.op(&Self::right_id()) == self
    }
}

/// A [`Magma`] is unital whenever it has a (unique) two-sided identity element.
pub trait Unital<Op: BinOpMarker>: LeftUnital<Op> + RightUnital<Op> {
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

impl<Op: BinOpMarker, T: LeftUnital<Op> + RightUnital<Op>> Unital<Op> for T {}
