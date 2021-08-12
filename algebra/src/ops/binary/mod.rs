//! Traits and types that represent operations.

pub mod impls;

use super::{unreachable, OpMarker};
use macros::*;
use std::marker::PhantomData;

/// A trait representing type markers for arbitrary binary operations. This
/// trait allows us to define multiple operations on the same types.
pub trait BinOpMarker: OpMarker {}

/// Declares a type marker for a binary operation.
macro_rules! decl_bin_op_marker {
    ($name:ident, $doc:literal) => {
        #[doc=$doc]
        #[derive(
            Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, OpMarker, BinOpMarker,
        )]
        pub struct $name;
    };
}

decl_bin_op_marker!(Add, "A type marker for addition.");
decl_bin_op_marker!(Sub, "A type marker for subtraction.");
decl_bin_op_marker!(Mul, "A type marker for multiplication.");
decl_bin_op_marker!(Div, "A type marker for division.");
decl_bin_op_marker!(Rem, "A type marker for remainders.");
decl_bin_op_marker!(BitAnd, "A type marker for bitwise and.");
decl_bin_op_marker!(BitOr, "A type marker for bitwise or.");
decl_bin_op_marker!(BitXor, "A type marker for bitwise xor.");
decl_bin_op_marker!(Shl, "A type marker for left bit shift.");
decl_bin_op_marker!(Shr, "A type marker for right bit shift.");

/// A trait that specifies that a given binary operation may be applied to two
/// types. This is where such binary operation is actually defined.
///
/// # Safety
/// There's **no guarantee** that this function won't invoke undefined behavior.
/// As such, this method should never be directly called, unless when
/// interfacing with a trait that makes stronger assertions about the behavior
/// of this function.
pub trait BinOp<Op: BinOpMarker, Rhs = Self> {
    /// The output of the binary operation.
    type Output;

    /// The error type of the checked method.
    type Err;

    /// Applies a binary operation on `self` and `rhs`.
    fn bin_op(&self, rhs: &Rhs) -> Result<Self::Output, Self::Err>;

    /// Applies a binary operation on `self` and `rhs`. Assumes that an error
    /// will never occur.
    ///
    /// # Safety
    /// If [`bin_op`] returns an error, this function is undefined behavior.
    unsafe fn bin_op_unchecked(&self, rhs: &Rhs) -> Self::Output {
        self.bin_op(rhs).unwrap_or_else(|_| unreachable())
    }
}

/// A trait that specifies that a given binary operation may be applied to two
/// types, and assigned to either. This is where such binary operation is
/// actually defined.
///
/// # Safety
/// There's **no guarantee** that this function won't invoke undefined behavior.
/// As such, this method should never be directly called, unless when
/// interfacing with a trait that makes stronger assertions about the behavior
/// of this function.
pub trait BinOpAssign<Op: BinOpMarker, Rhs = Self> {
    /// The error type of the checked method.
    type Err;

    /// Applies a binary operation on `self` and `rhs`, assigns it to `self`.
    fn bin_op_assign_lhs(&mut self, rhs: &Rhs) -> Result<(), Self::Err>;

    /// Applies a binary operation on `self` and `rhs`, assigns it to `rhs`.
    fn bin_op_assign_rhs(&self, rhs: &mut Rhs) -> Result<(), Self::Err>;

    /// Applies a binary operation on `self` and `rhs`, assigns it to `self`.
    /// Assumes that an error will never occur.
    ///
    /// # Safety
    /// If [`bin_op_assign_lhs`] returns an error, this function is undefined
    /// behavior.
    unsafe fn bin_op_assign_lhs_unchecked(&mut self, rhs: &Rhs) {
        self.bin_op_assign_lhs(rhs)
            .unwrap_or_else(|_| unreachable())
    }

    /// Applies a binary operation on `self` and `rhs`, assigns it to `rhs`.
    /// Assumes that an error will never occur.
    ///
    /// # Safety
    /// If [`bin_op_assign_rhs`] returns an error, this function is undefined
    /// behavior.
    unsafe fn bin_op_assign_rhs_unchecked(&self, rhs: &mut Rhs) {
        self.bin_op_assign_rhs(rhs)
            .unwrap_or_else(|_| unreachable())
    }
}

/// Represents a "backwards" operator, so that `a op b = b Bws(op) a`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bws<Op: BinOpMarker>(PhantomData<Op>);
impl<Op: BinOpMarker> OpMarker for Bws<Op> {}
impl<Op: BinOpMarker> BinOpMarker for Bws<Op> {}

impl<Op: BinOpMarker, Lhs, Rhs: BinOp<Op, Lhs>> BinOp<Bws<Op>, Rhs> for Lhs {
    type Output = Rhs::Output;
    type Err = Rhs::Err;

    fn bin_op(&self, rhs: &Rhs) -> Result<Self::Output, Self::Err> {
        rhs.bin_op(self)
    }
}

impl<Op: BinOpMarker, Lhs, Rhs: BinOp<Op, Lhs>> BinOpAssign<Bws<Op>, Rhs> for Lhs {
    type Err = Rhs::Err;

    fn bin_op_assign_lhs(&mut self, rhs: &Rhs) -> Result<(), Self::Err> {
        rhs.bin_op_assign_rhs(self)
    }

    fn bin_op_assign_rhs(&self, rhs: &mut Rhs) -> Result<(), Self::Err> {
        rhs.bin_op_assign_lhs(self)
    }
}

pub type BwsSub = Bws<Sub>;
pub type BwsDiv = Bws<Div>;
