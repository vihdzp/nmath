pub mod impls;

use super::Magma;
use crate::{numbers::naturals::Naturals, ops::BinOpMarker};

/// A [power associative](https://en.wikipedia.org/wiki/Power_associativity)
/// magma is one for which repeated application of an operation is uniquely
/// defined.
///
/// It is the implementor's responsability to ratify that power associativity
/// holds in a given data structure.
pub trait PowAssociative<Op: BinOpMarker>: Magma<Op> {
    /// Evaluates `b op b op ... b`, applied `e` times, and assigns it to
    /// `self`. Uses exponentiation by squares. If `e` is zero, leaves `self`
    /// unchanged.
    fn pow_p<N: Naturals>(&mut self, mut e: N)
    where
        Self: Clone,
    {
        if e.le_prim(1u32) {
            return;
        }

        let mut x = self.clone();

        while e.gt_prim(1u32) {
            if e.is_odd() {
                self.op_assign_lhs(&x);
            }

            x.op_assign_lhs(&x.clone());
            e.div_assign_prim(2u32);
        }
    }
}

/// A [`Magma`] is left alternative whenever `(a op a) op b = a op (a op b)` for
/// all `a, b`.
///
/// It is the implementor's responsability to ratify that left alternativity
/// holds in a given data structure.
pub trait LeftAlternative<Op: BinOpMarker>: Magma<Op> {
    /// Tests whether `(a op a) op b = a op (a op b)` for a given set of entries.
    fn test_left_altern(&self, rhs: &Self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = self.op(self);
        lhs.op_assign_lhs(rhs);

        let mut rhs = self.op(rhs);
        self.op_assign_rhs(&mut rhs);

        lhs == rhs
    }
}

/// A [`Magma`] is right alternative whenever `(a op b) op b = a op (b op b)`
/// for all `a, b`.
///
/// It is the implementor's responsability to ratify that right alternativity
/// holds in a given data structure.
pub trait RightAlternative<Op: BinOpMarker>: Magma<Op> {
    /// Tests whether `(a op a) op b = a op (a op b)` for a given set of entries.
    fn test_right_altern(&self, rhs: &Self) -> bool
    where
        Self: PartialEq,
    {
        let mut lhs = self.op(rhs);
        lhs.op_assign_lhs(rhs);

        let mut rhs = rhs.op(rhs);
        self.op_assign_rhs(&mut rhs);

        lhs == rhs
    }
}

/// A [`Magma`] is alternative whenever it's both [`LeftAlternative`] and
/// [`RightAlternative`].
///
/// It is the implementor's responsability to ratify that alternativity holds in
/// a given data structure.
///
/// **Note:** [`Alternative`] doesn't imply [`PowAssociative`]. See
/// [https://math.stackexchange.com/a/3132319].
pub trait Alternative<Op: BinOpMarker>: LeftAlternative<Op> + RightAlternative<Op> {}

impl<Op: BinOpMarker, T: LeftAlternative<Op> + RightAlternative<Op>> Alternative<Op> for T {}

/// A [`Magma`] is [associative](https://en.wikipedia.org/wiki/Associative_property)
/// whenever `(a op b) op c = a op (b op c)` for all `a, b, c`.
///
/// It is the implementor's responsability to ratify that associativity holds
/// in a given data structure.
pub trait Associative<Op: BinOpMarker>: PowAssociative<Op> + Alternative<Op> {
    /// Tests whether `(a op b) op c = a op (b op c)` for a given set of entries.
    fn test_assoc(a: &Self, b: &Self, c: &Self) -> bool
    where
        Self: PartialEq,
    {
        let mut x = Self::op(a, b);
        x.op_assign_lhs(c);

        let mut y = Self::op(b, c);
        a.op_assign_rhs(&mut y);

        x == y
    }
}

/// Implements [`PowAssociative`], [`LeftAlternative`], and [`RightAlternative`]
/// for a type.
#[macro_export]
macro_rules! impl_associativity {
    ($op:ty: $type:ty) => {
        impl crate::PowAssociative<$op> for $type {}
        impl crate::LeftAlternative<$op> for $type {}
        impl crate::RightAlternative<$op> for $type {}
    };
}
