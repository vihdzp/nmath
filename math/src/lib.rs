//! Implements various numerical types.

pub mod permutation;
pub mod storage;

/// A wrapper for wrapping operations. This amounts to taking a numerical type
/// modulo some power of two.
pub type W<T> = core::num::Wrapping<T>;

/// A wrapper for unsigned integers, which states they are to be treated as
/// [natural numbers](https://en.wikipedia.org/wiki/Natural_number). This has
/// the following implications:
///
/// - Naturals are a [`Group`] under addition and a [`Monoid`] under
///   multiplication, and the same will be assumed of this wrapper type. That
///   is to say, we'll assume **no overflow** occurs.
/// - One can't flip the digits of an infinite binary expansion, so we won't
///   implement [`Not`] for this type.
pub struct N<T>(T);

/// A wrapper for floating point numbers, which states they are to be treated
/// as [real numbers](https://en.wikipedia.org/wiki/Real_number). This has the
/// following implications:
///
/// - Equality is evaluated using relative differences, so that addition and
///   multiplication may be associative. This means equality is not technically
///   transitive, but we will still treat it as such. Be wary of this.
/// - Comparison is an order relation, which means that the inner type will be
///   assumed to not be `NaN`.
/// - Reals are unbounded, which means we'll disallow `Infinity` and
///   `-Infinity`.
pub struct R<T>(T);

/// A wrapper for floating point numbers, which states they are to be treated
/// as [projectively extended real numbers](https://en.wikipedia.org/wiki/Projectively_extended_real_line).
/// This has the following implications:
///
/// - Equality is evaluated using relative differences, so that addition and
///   multiplication may be associative. This means equality is not technically
///   transitive, but we will still treat it as such. Be wary of this.
/// - Values of `Infinity` and `-Infinity` will be treated as equal.
/// - Ordering is partial, but `Infinity` will compare as both smaller and
///   larger than any other number except itself.
/// - The inner type will be assumed to not be `NaN`.
/// - [`Rec`] is an [`Automorphism`] of the type.
pub struct Rp<T>(T);

/// A wrapper for floating point numbers, which states they are to be treated
/// as [affinely extended real numbers](https://en.wikipedia.org/wiki/Extended_real_number_line).
/// This has the following implications:
///
/// - Equality is evaluated using relative differences, so that addition and
///   multiplication may be associative. This means equality is not technically
///   transitive, but we will still treat it as such. Be wary of this.
/// - Ordering is total, which means that the inner type will be assumed to not
///   be `NaN`.
pub struct Ra<T>(T);
