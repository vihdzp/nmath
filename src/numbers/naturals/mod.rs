use crate::{
    algebra::{Commutative, Monoid, Unital},
    ops::{Add, BinOp, Mul},
};

pub trait Naturals: Monoid<Add> + Monoid<Mul> + Commutative<Add> + Commutative<Mul>
where
    for<'a> &'a Self: BinOp<Add, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Add, &'b Self, Output = Self>,
    for<'a> &'a Self: BinOp<Mul, Self, Output = Self>,
    for<'a, 'b> &'a Self: BinOp<Mul, &'b Self, Output = Self>,
{
    /// Returns the zero element.
    fn zero() -> Self {
        <Self as Unital<Add>>::id()
    }

    /// Returns the one element.
    fn one() -> Self {
        <Self as Unital<Mul>>::id()
    }
}
