//! Defines algebraic structures.

mod group;
mod involution;
mod numbers;
mod ops;

pub use group::*;
pub use involution::*;
pub use numbers::*;
pub use ops::*;

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

#[inline(always)]
/// Unwraps a result. Unsafely assumes that this value is always an `Ok`.
///
/// # Safety
/// This function is undefined behavior if the value is of type `Err`.
unsafe fn unwrap_unchecked<T, U>(res: Result<T, U>) -> T {
    res.unwrap_or_else(|_| unreachable())
}
