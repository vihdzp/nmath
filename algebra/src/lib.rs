//! Defines algebraic structures.

mod absorbing;
mod associative;
mod commutative;
mod r#loop;
mod magma;
mod monoid;
mod numbers;
mod ops;
mod quasigroup;
mod unital;

pub use absorbing::*;
pub use associative::*;
pub use commutative::*;
pub use magma::*;
pub use monoid::*;
pub use numbers::*;
pub use ops::*;
pub use quasigroup::*;
pub use r#loop::*;
pub use unital::*;

use crate::ops::BinOpMarker;

/// A [group](https://en.wikipedia.org/wiki/Group_(mathematics)) is
/// [`Associative`] and a [`Loop`]
pub trait Group<Op: BinOpMarker>: Associative<Op> + Loop<Op> {}

impl<Op: BinOpMarker, T: Unital<Op> + Quasigroup<Op> + Associative<Op>> Group<Op> for T {}
