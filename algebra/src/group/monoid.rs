use super::{Associative, Commutative, PowAssociative, Unital};
use crate::{numbers::Naturals, ops::BinOpMarker};

/// A [`Unital`] and [`PowAssociative`] structure.
pub trait UnitalPowAssociative<Op: BinOpMarker>: Unital<Op> + PowAssociative<Op> {
    /// Evaluates `b op b op ... b`, applied `e` times, and assigns it to
    /// `self`. Uses exponentiation by squares. If `e = 0`, returns the identity.
    ///
    /// If you don't care about the case where `e = 0`, consider using [`pow_p`].
    fn pow_z<N: Naturals>(&mut self, e: N)
    where
        Self: Clone,
    {
        if e.is_zero() {
            *self = Self::id();
        } else {
            self.pow_p(e);
        }
    }
}

impl<Op: BinOpMarker, T: Unital<Op> + PowAssociative<Op>> UnitalPowAssociative<Op> for T {}

/// A [monoid](https://en.wikipedia.org/wiki/Monoid) is both [`Unital`] and
/// [`Associative`].
pub trait Monoid<Op: BinOpMarker>: Unital<Op> + Associative<Op> {}

impl<Op: BinOpMarker, T: Unital<Op> + Associative<Op>> Monoid<Op> for T {}

/// A [`Monoid`] that is also [`Commutative`].
pub trait CommutativeMonoid<Op: BinOpMarker>: Monoid<Op> + Commutative<Op> {}

impl<Op: BinOpMarker, T: Monoid<Op> + Commutative<Op>> CommutativeMonoid<Op> for T {}
