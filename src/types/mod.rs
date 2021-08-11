//! Implements many types that can be used throughout the library.

/// A wrapper around floating points that treats them as real numbers. This has
/// various implications.
///
/// - Floating points are now considered as being associative under addit
pub struct R32(f32);