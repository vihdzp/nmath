pub mod binary;
pub mod unary;

pub use binary::*;
pub use unary::*;

/// A trait representing type markers for n-ary operations.
pub trait OpMarker {}
