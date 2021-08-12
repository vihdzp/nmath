pub mod impls;

use super::Magma;
use crate::ops::BinOpMarker;

/// A trait for a [`Magma`] with a unique [left-absorbing](https://en.wikipedia.org/wiki/Absorbing_element)
/// element, satisfying `e op x = e` for any `x`.
pub trait LeftAbsorbing<Op: BinOpMarker>: Magma<Op> {
    /// Returns the unique left-absorbing element of the magma.
    fn left_absorb() -> Self;

    /// Tests whether the left zero times a given value equals the zero.
    fn test_left_absorb(&self) -> bool
    where
        Self: PartialEq,
    {
        let left_abs = Self::left_absorb();
        Self::left_absorb().op(self) == left_abs
    }
}

/// A trait for a [`Magma`] with a unique [right-absorbing](https://en.wikipedia.org/wiki/Absorbing_element)
/// element, satisfying `x op e = e` for any `x`.
pub trait RightAbsorbing<Op: BinOpMarker>: Magma<Op> {
    /// Returns the unique right-absorbing element of the magma.
    fn right_absorb() -> Self;

    /// Tests whether a given value times the right zero equals the zero.
    fn test_right_absorb(&self) -> bool
    where
        Self: PartialEq,
    {
        let right_abs = Self::right_absorb();
        self.op(&right_abs) == right_abs
    }
}

/// A trait for a [`Magma`] with a unique [absorbing](https://en.wikipedia.org/wiki/Absorbing_element)
/// element, satisfying `e op x = e` and `x op e = e` for any `x`.
pub trait Absorbing<Op: BinOpMarker>: LeftAbsorbing<Op> + RightAbsorbing<Op> {
    /// Returns the unique absorbing element of the magma.
    fn absorb() -> Self {
        Self::left_absorb()
    }
}

impl<Op: BinOpMarker, T: LeftAbsorbing<Op> + RightAbsorbing<Op>> Absorbing<Op> for T {}
