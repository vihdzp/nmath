mod impls;

pub use impls::*;

use super::{unreachable, OpMarker};
use macros::*;

/// A trait representing type markers for arbitrary unary operations. This trait
/// allows us to define multiple operations on the same types.
pub trait UnOpMarker: OpMarker {}

/// Declares a type marker for a unary operation.
macro_rules! decl_un_op_marker {
    ($name:ident, $doc:literal) => {
        #[doc=$doc]
        #[derive(
            Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, OpMarker, UnOpMarker,
        )]
        pub struct $name;
    };
}

decl_un_op_marker!(Neg, "A type marker for negation.");
decl_un_op_marker!(Rec, "A type marker for reciprocals.");
decl_un_op_marker!(Not, "A type marker for logical negation.");

/// A trait that specifies that a given unary operation may be applied to a
/// type. This is where such unary operation is actually defined.
///
/// # Safety
/// There's **no guarantee** that this function won't invoke undefined behavior.
/// As such, this method should never be directly called, unless when
/// interfacing with a trait that makes stronger assertions about the behavior
/// of this function.
trait UnOp<Op: UnOpMarker> {
    /// The output of the unary operation.
    type Output;

    /// The error type of the checked method.
    type Err;

    /// Applies a unary operation on `self`.
    fn un_op(&self) -> Result<Self::Output, Self::Err>;

    /// Applies a unary operation on `self`. Assumes that an error will never
    /// occur.
    ///
    /// # Safety
    /// If [`un_op`] returns an error, this function is undefined behavior.
    unsafe fn un_op_unchecked(&self) -> Self::Output {
        self.un_op().unwrap_or_else(|_| unreachable())
    }
}

/// A trait that specifies that a given unary operation may be applied to a type
/// and assigned to it.
pub trait UnOpAssign<Op: UnOpMarker, Rhs = Self> {
    /// The error type of the checked method.
    type Err;

    /// Applies a unary operation on `self` and assigns the result.
    fn un_op_assign(&mut self) -> Result<(), Self::Err>;

    /// Applies a unary operation on `self` and assigns the result.
    ///
    /// # Safety
    /// If [`un_op_assign`] returns an error, this function is undefined
    /// behavior.
    unsafe fn un_op_assign_unchecked(&mut self) {
        self.un_op_assign().unwrap_or_else(|_| unreachable())
    }
}
