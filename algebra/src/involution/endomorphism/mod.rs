use crate::{UnOpMarker, UnOpSet};

pub mod impls;

/// Mathematically, an [endomorphism](https://en.wikipedia.org/wiki/Endomorphism)
/// is a set together with a closed unary operation. No further restrictions are
/// imposed.
///
/// # Safety
/// In contrast to [`UnOpSet`], we require that an endomorphism **does not invoke
/// UB** under any "normal" circumstances. The implementor is free to choose
/// whatever a "normal" circumstance means, but they must be aware of the
/// consequences it entails.
pub trait Endomorphism<Op: UnOpMarker>: UnOpSet<Op> {
    /// Applies a unary operation on a value.
    ///
    /// On non-copy types, this will often create a new buffer.
    fn op(&self) -> Self {
        unsafe { self.un_op_unchecked() }
    }

    /// Applies a binary operation on a value, assigns the result to itself.
    fn op_assign(&mut self) {
        unsafe {
            self.un_op_assign_unchecked();
        }
    }
}
