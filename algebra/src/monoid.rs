use super::{Associative, Commutative, Unital};
use crate::ops::BinOpMarker;

/// A [monoid](https://en.wikipedia.org/wiki/Monoid) is both [`Unital`] and
/// [`Associative`].
pub trait Monoid<Op: BinOpMarker>: Unital<Op> + Associative<Op> {}

impl<Op: BinOpMarker, T: Unital<Op> + Associative<Op>> Monoid<Op> for T {}

/// A [`Monoid`] that is also [`Commutative`].
pub trait CommutativeMonoid<Op: BinOpMarker>: Monoid<Op> + Commutative<Op> {}

impl<Op: BinOpMarker, T: Monoid<Op> + Commutative<Op>> CommutativeMonoid<Op> for T {}
