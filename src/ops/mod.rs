//! Traits and types that represent operations.

pub mod impls;

use core::ops;
use std::marker::PhantomData;

/// A trait representing type markers for n-ary operations.
pub trait OpMarker {}

/// Declares a type marker for an n-ary operation.
macro_rules! decl_op_marker {
    ($name:ident, $n_marker:ident, $doc:literal) => {
        #[doc=$doc]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name;
        impl OpMarker for $name {}
        impl $n_marker for $name {}
    };
}

/// A trait representing type markers for arbitrary unary operations. This trait
/// allows us to define multiple operations on the same types.
pub trait UnOpMarker: OpMarker {}

/// Declares a type marker for a binary operation.
macro_rules! decl_un_op_marker {
    ($name:ident, $doc:literal) => {
        decl_op_marker!($name, UnOpMarker, $doc);
    };
}

decl_un_op_marker!(Neg, "A type marker for negation.");
decl_un_op_marker!(Rec, "A type marker for reciprocals.");
decl_un_op_marker!(Not, "A type marker for logical negation.");

/// A trait that specifies that a given unary operation may be applied to a
/// type.
trait UnOp<Op: UnOpMarker> {
    type Output;

    /// Applies a unary operation on `self`.
    fn un_op(self) -> Self::Output;
}

/// A trait that specifies that a given unary operation may be applied to a type
/// and assigned to it.
pub trait UnOpAssign<Op: UnOpMarker, Rhs = Self> {
    /// Applies a unary operation on `self` and assigns the result.
    fn un_op_assign(&mut self);
}

/// A trait representing type markers for arbitrary binary operations. This
/// trait allows us to define multiple operations on the same types.
pub trait BinOpMarker: OpMarker {}

/// Declares a type marker for a binary operation.
macro_rules! decl_bin_op_marker {
    ($name:ident, $doc:literal) => {
        decl_op_marker!($name, BinOpMarker, $doc);
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
/// types.
pub trait BinOp<Op: BinOpMarker, Rhs = Self> {
    type Output;

    /// Applies a binary operation on `self`.
    fn bin_op(self, rhs: Rhs) -> Self::Output;
}

/// A trait that specifies that a given binary operation may be applied to two
/// types and assigned to the former.
pub trait BinOpAssign<Op: BinOpMarker, Rhs = Self> {
    /// Applies a binary operation on `self` and assigns the result.
    fn bin_op_assign(&mut self, rhs: Rhs);
}

/// Represents a "backwards" operator, so that `a Op b = b Bws(Op) a`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bws<Op: BinOpMarker>(PhantomData<Op>);
impl<Op: BinOpMarker> OpMarker for Bws<Op> {}
impl<Op: BinOpMarker> BinOpMarker for Bws<Op> {}

impl<Op: BinOpMarker, Lhs, Rhs: BinOp<Op, Lhs>> BinOp<Bws<Op>, Rhs> for Lhs {
    type Output = Rhs::Output;

    fn bin_op(self, rhs: Rhs) -> Self::Output {
        rhs.bin_op(self)
    }
}

pub type BwsSub = Bws<Sub>;
pub type BwsDiv = Bws<Div>;
