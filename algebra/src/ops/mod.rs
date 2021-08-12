pub mod binary;
pub mod unary;

pub use binary::*;
pub use unary::*;

/// A trait representing type markers for n-ary operations.
pub trait OpMarker {}

#[cold]
#[inline(always)]
/// A function representing unreachable code.
fn unreachable() -> ! {
    if cfg!(debug_assertions) {
        unreachable!()
    } else {
        unsafe { std::hint::unreachable_unchecked() }
    }
}
