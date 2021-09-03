//! All of the traits relating to a [`Group`].
//!
//! The hierarchy of traits for this type is as follows:
//!
//! ```ignore
//!                                  +-----+                                 
//!       +---------------+----------|Magma|----------+---------------+      
//!       |               |          +-----+          |               |      
//!       |               |             |             |               |      
//!       v               v             v             v               v      
//! +-----------+   +----------+    +-------+   +-----------+   +-----------+
//! |Absorbing  |   |Quasigroup|    |Unital |   |Associative|   |Commutative|
//! +-----------+   +----------+    +-------+   +-----------+   +-----------+
//!                       |             |             |                      
//!                       v             |             v                      
//!                    +-----+          |         +-------+                  
//!                    |Loop |<---------+-------->|Monoid |                  
//!                    +-----+                    +-------+                  
//!                       |                           |                      
//!                       |                           |                      
//!                       |          +-----+          |                      
//!                       +--------->|Group| <--------+                      
//!                                  +-----+                                 
//! ```

mod absorbing;
mod associative;
mod commutative;
mod r#loop;
mod magma;
mod monoid;
mod quasigroup;
mod unital;

pub use absorbing::*;
pub use associative::*;
pub use commutative::*;
pub use magma::*;
pub use monoid::*;
pub use quasigroup::*;
pub use r#loop::*;
pub use unital::*;

use crate::{BinOpMarker, UnOpMarker};

/// A [group](https://en.wikipedia.org/wiki/Group_(mathematics)) is
/// [`Associative`] and a [`Loop`]
pub trait Group<Op: BinOpMarker>: Associative<Op> + Loop<Op> {
    /// The inverse operator.
    type Inv: UnOpMarker;

    fn inv(&self) -> Self {
        todo!()
    }
}
