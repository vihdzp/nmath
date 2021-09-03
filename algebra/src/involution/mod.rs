mod endomorphism;

pub use endomorphism::*;

use crate::{UnOp, UnOpAssign, UnOpMarker, UnOpSet};

/// An [injective function](https://en.wikipedia.org/wiki/Injective_function)
/// is such that `op a = op b` implies `a = b`.
pub trait Injective<Op: UnOpMarker>: Endomorphism<Op> + UnOpSet<Self::Inv> {
    /// The (partial) inverse operator of `op`, satisfying `Inv (op a) = a` for
    /// any `a`.
    type Inv: UnOpMarker;

    /// Whenever `self` is of the form `op a`, returns the unique possible value
    /// of `a`. Returns an error otherwise.
    fn partial_inv(&self) -> Result<Self, <Self as UnOp<Self::Inv>>::Err> {
        <Self as UnOp<Self::Inv>>::un_op(self)
    }

    /// Whenever `self` is of the form `op a`, returns the unique possible value
    /// of `a`.
    ///
    /// # Safety
    /// If `self` is not of the form `op a`, this method makes no safety
    /// guarantees nor guarantees about the result.
    unsafe fn partial_inv_unchecked(&self) -> Self {
        <Self as UnOp<Self::Inv>>::un_op_unchecked(self)
    }

    /// Whenever `self` is of the form `op a`, evaluates the unique possible
    /// value of `a` and assigns it to `self`. Returns an error otherwise.
    fn partial_inv_assign(&mut self) -> Result<(), <Self as UnOpAssign<Self::Inv>>::Err> {
        <Self as UnOpAssign<Self::Inv>>::un_op_assign(self)
    }

    /// Whenever `self` is of the form `op a`, evaluates the unique possible
    /// value of `a` and assigns it to `self`.
    ///
    /// # Safety
    /// If `self` is not of the form `op a`, this method makes no safety
    /// guarantees nor guarantees about the result.
    unsafe fn partial_inv_assign_unchecked(&mut self) {
        <Self as UnOpAssign<Self::Inv>>::un_op_assign_unchecked(self)
    }

    /// Tests that the inverse of a function applied on a value equals the
    /// value.
    ///
    /// # Panics
    /// This function should panic if the inverse function fails.
    fn test_inv(&self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = self.op();
        lhs.partial_inv_assign().unwrap();
        &lhs == self
    }

    /// Tests that the inverse of a function applied on a value equals the
    /// value.
    ///
    /// # Safety
    /// If the partial inverse function fails, this function is undefined
    /// behavior.
    unsafe fn test_inv_unchecked(&self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = self.op();
        lhs.partial_inv_assign_unchecked();
        &lhs == self
    }
}

/// A [surjective function](https://en.wikipedia.org/wiki/Surjective_function)
/// is such that for every `a` there exists `b` such that `op a = b`.
pub trait Surjective<Op: UnOpMarker>: Endomorphism<Op> {}

/// An [automorphism](https://en.wikipedia.org/wiki/Automorphism) is an
/// [`Endomorphism`] that is both [`Injective`] and [`Surjective`]. This implies
/// the existence of an inverse operator that must be both [`Injective`] and
/// [`Surjective`] as well.
pub trait Automorphism<Op: UnOpMarker>:
    Injective<Op>
    + Surjective<Op>
    + Injective<<Self as Injective<Op>>::Inv, Inv = Op>
    + Surjective<<Self as Injective<Op>>::Inv>
{
    /// The main operation on an [`Automorphism`].
    fn auto_op(&self) -> Self {
        <Self as Endomorphism<Op>>::op(self)
    }

    /// The main operation-assign on an [`Automorphism`].
    fn auto_op_assign(&mut self) {
        <Self as Endomorphism<Op>>::op_assign(self)
    }

    /// The inverse operation on an [`Automorphism`].
    fn inv(&self) -> Self {
        unsafe { <Self as Injective<Op>>::partial_inv_unchecked(self) }
    }

    /// The inverse operation-assign on an [`Automorphism`].
    fn inv_assign(&mut self) {
        unsafe { <Self as Injective<Op>>::partial_inv_assign_unchecked(self) }
    }

    /// Tests that the inverse of a function applied on a value equals the
    /// value, and viceversa.
    fn test_inv(&self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = self.auto_op();
        lhs.inv_assign();

        if &lhs != self {
            return false;
        }

        let mut lhs = self.inv();
        lhs.auto_op_assign();
        &lhs == self
    }
}

/// An involution is an [`Automorphism`] that gives the same result when applied
/// twice.
pub trait Involution<Op: UnOpMarker>: Automorphism<Op> {
    /// The main operation on an [`Involution`].
    fn involution_op(&self) -> Self {
        self.auto_op()
    }

    /// The main operation-assign on an [`Involution`].
    fn involution_op_assign(&mut self) {
        self.auto_op_assign()
    }

    /// Tests that the function of a value is the same as its inverse.
    fn test_involution(&self) -> bool
    where
        Self: PartialEq,
    {
        self.auto_op() == self.inv()
    }
}
