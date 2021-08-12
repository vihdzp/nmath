use std::cmp;

use crate::{Add, BinOp, CommutativeMonoid, Magma, Mul, Unital};

pub trait PrimitiveNaturals:
    Copy + From<u8> + From<u16> + From<u32> + From<u64> + From<u128>
{
}

/// An auxiliary trait that makes implementing [`Naturals`] easier.
pub trait NaturalsAux<Rhs, Output>:
    BinOp<Add, Rhs, Output = Output> + BinOp<Mul, Rhs, Output = Output>
{
}

impl<Rhs, Output, T> NaturalsAux<Rhs, Output> for T where
    T: BinOp<Add, Rhs, Output = Output> + BinOp<Mul, Rhs, Output = Output>
{
}

pub trait Naturals:
    CommutativeMonoid<Add>
    + CommutativeMonoid<Mul>
    //+ ArithOp<Self::Word, Output = Self>
   // + ArithOpAssign<Self::Word>
    + From<Self::Word>
{
    /// The preferred primitive naturals to interface with these numbers.
    type Word: PrimitiveNaturals; //+ ArithOp<Self, Output = Self>;

    /// Returns the zero element.
    fn zero() -> Self {
        <Self as Unital<Add>>::id()
    }

    /// Returns the one element.
    fn one() -> Self {
        <Self as Unital<Mul>>::id()
    }

    /// Compares `self` to a primitive.
    fn cmp_prim<N: Into<Self::Word>>(&self, n: N) -> cmp::Ordering;

    /// Determines whether `self` equals a primitive.
    fn eq_prim<N: Into<Self::Word>>(&self, n: N) -> bool {
        self.cmp_prim(n).is_eq()
    }

    /// Determines whether `self` does not equal a primitive.
    fn ne_prim<N: Into<Self::Word>>(&self, n: N) -> bool {
        self.cmp_prim(n).is_ne()
    }

    /// Determines whether `self` is less than a primitive.
    fn lt_prim<N: Into<Self::Word>>(&self, n: N) -> bool {
        self.cmp_prim(n).is_lt()
    }

    /// Determines whether `self` is less or equal to a primitive.
    fn le_prim<N: Into<Self::Word>>(&self, n: N) -> bool {
        self.cmp_prim(n).is_le()
    }

    /// Determines whether `self` is greater than a primitive.
    fn gt_prim<N: Into<Self::Word>>(&self, n: N) -> bool {
        self.cmp_prim(n).is_gt()
    }

    /// Determines whether `self` is greater or equal to a primitive.
    fn ge_prim<N: Into<Self::Word>>(&self, n: N) -> bool {
        self.cmp_prim(n).is_ge()
    }

    /// Returns whether `self` is even.
    fn is_even(&self) -> bool;

    /// Returns whether `self` is odd.
    fn is_odd(&self) -> bool {
        !self.is_even()
    }

    /// Adds two values.
    fn add(&self,rhs:&Self) -> Self {
        <Self as Magma<Add>>::op(self, rhs)
    }

    /// Subtracts two values.
    fn sub(&self, _rhs:&Self) -> Self {
        todo!()
    }

    fn mul_assign(&mut self, _rhs:&Self){}

    /// Divides `self` by a primitive.
    fn div_assign_prim<N: Into<Self::Word>>(&mut self, _n: N) {
     todo!()
    }
}
