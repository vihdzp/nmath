//! Defines algebraic structures.

mod associative;
mod commutative;
mod quasigroup;
mod unital;

use std::{borrow::Borrow,  sync::Arc};

pub use associative::Associative;
pub use commutative::Commutative;
pub use quasigroup::Quasigroup;
pub use unital::Unital;

use crate::ops::{BinOp, BinOpAssign, BinOpMarker};

/// A trait for a value that might either be owned or immutably borrowed.
pub trait MaybeRef<T>: Sized + Borrow<T> {
    /// Whether the value is owned or not.
    const OWNED: bool;

    /// Attempts to take the value as owned.
    ///
    /// # Panics
    /// This will panic if called on a reference.
    fn into_owned(self) -> T {
        panic!("called into_owned on a reference")
    }
}

impl<T> MaybeRef<T> for T {
    const OWNED: bool = true;

    fn into_owned(self) -> T {
        self
    }
}

impl<T> MaybeRef<T> for Box<T> {
    const OWNED: bool = true;

    fn into_owned(self) -> T {
        *self
    }
}

impl<'a, T> MaybeRef<T> for &'a T {
    const OWNED: bool = false;
}

impl<'a, T> MaybeRef<T> for &'a mut T {
    const OWNED: bool = false;
}

impl<T> MaybeRef<T> for Arc<T> {
    const OWNED: bool = false;
}

/// Mathematically, a [magma](https://en.wikipedia.org/wiki/Magma_(algebra)) is
/// a set together with a closed binary operation. No further restrictions are
/// imposed.
///
/// We require that references to values can also be operated on, so that we
/// don't need to clone values needlessly. We furthermore require the
/// implementation of assigning operations, so that we may use them whenever
/// they're more efficient than their non-assigning counterparts.
// TODO: macro for easier implementation?
pub trait Magma<Op: BinOpMarker>:
    Sized
    + BinOp<Op, Output = Self>
    + for<'a> BinOp<Op, &'a Self, Output = Self>
    + BinOpAssign<Op>
    + for<'a> BinOpAssign<Op, &'a Self>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
    /// Applies a binary operation on two values, which are taken as owned
    /// whenever possible.
    fn op<T: MaybeRef<Self>, U: MaybeRef<Self>>(lhs: T, rhs: U) -> Self {
        // All of these ifs should disappear at compile time.
        if T::OWNED {
            if U::OWNED {
                lhs.into_owned().bin_op(rhs.into_owned())
            } else {
                lhs.into_owned().bin_op(rhs.borrow())
            }
        } else if U::OWNED {
            lhs.borrow().bin_op(rhs.into_owned())
        } else {
            lhs.borrow().bin_op(rhs.borrow())
        }
    }
}

impl<Op: BinOpMarker, T> Magma<Op> for T
where
    T: Sized
        + BinOp<Op, Output = Self>
        + for<'a> BinOp<Op, &'a Self, Output = Self>
        + BinOpAssign<Op>
        + for<'a> BinOpAssign<Op, &'a Self>,
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
}

/// A [monoid](https://en.wikipedia.org/wiki/Monoid) is both [`Unital`] and
/// [`Associative`].
pub trait Monoid<Op: BinOpMarker>: Unital<Op> + Associative<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
}

impl<Op: BinOpMarker, T: Unital<Op> + Associative<Op>> Monoid<Op> for T
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>,
{
}

/// A [loop](https://en.wikipedia.org/wiki/Quasigroup#Loops) is both [`Unital`]
/// and a [`Quasigroup`].
pub trait Loop<Op: BinOpMarker>: Unital<Op> + Quasigroup<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>
        + BinOp<Self::LInv, Self, Output = Self>
        + BinOp<Self::RInv, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>
        + BinOp<Self::LInv, &'b Self, Output = Self>
        + BinOp<Self::RInv, &'b Self, Output = Self>,
{
    /// Returns the left inverse of a value.
    fn left_inv<T: MaybeRef<Self>>(value: T) -> Self {
        <Self as Magma<Self::RInv>>::op(Self::id(), value)
    }

    /// Returns the right inverse of a value.
    fn right_inv<T: MaybeRef<Self>>(value: T) -> Self {
        <Self as Magma<Self::LInv>>::op(Self::id(), value)
    }
}

impl<Op: BinOpMarker, T: Unital<Op> + Quasigroup<Op>> Loop<Op> for T
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>
        + BinOp<Self::LInv, Self, Output = Self>
        + BinOp<Self::RInv, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>
        + BinOp<Self::LInv, &'b Self, Output = Self>
        + BinOp<Self::RInv, &'b Self, Output = Self>,
{
}

/// A [group](https://en.wikipedia.org/wiki/Group_(mathematics)) is
/// [`Associative`] and a [`Loop`]
pub trait Group<Op: BinOpMarker>: Associative<Op> + Loop<Op>
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>
        + BinOp<Self::LInv, Self, Output = Self>
        + BinOp<Self::RInv, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>
        + BinOp<Self::LInv, &'b Self, Output = Self>
        + BinOp<Self::RInv, &'b Self, Output = Self>,
{
}

impl<Op: BinOpMarker, T: Unital<Op> + Quasigroup<Op> + Associative<Op>> Group<Op> for T
where
    for<'a> &'a Self: BinOp<Op, Self, Output = Self>
        + BinOp<Self::LInv, Self, Output = Self>
        + BinOp<Self::RInv, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Op, &'b Self, Output = Self>
        + BinOp<Self::LInv, &'b Self, Output = Self>
        + BinOp<Self::RInv, &'b Self, Output = Self>,
{
}
