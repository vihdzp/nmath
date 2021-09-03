use crate::storage::{OwnedStorage, Storage};

/// The parity of a permutation.
pub enum Parity {
    Even,
    Odd,
}

pub struct Permutation<S: Storage<Inner = usize>>(S);

impl<S: OwnedStorage<Inner = usize>> Permutation<S> {
    pub fn parity(&self) -> Parity {
        todo!()
    }
}
