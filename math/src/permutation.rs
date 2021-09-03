use std::{
    iter::{self, FromIterator},
    ops::{Index, IndexMut},
};

use crate::storage::{
    ArrayStorageE, ContiguousStorage, ContiguousStorageMut, Iter, IterMut, OwnedStorage, Size,
    Storage, StorageMut, VecStorageE,
};

/// The parity of a permutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parity {
    /// Even parity.
    Even,

    /// Odd parity.
    Odd,
}

impl Parity {
    /// Flips the parity of `self`.
    #[must_use = "use `flip_mut` to flip the parity in place"]
    pub fn flip(&self) -> Self {
        match self {
            Self::Even => Self::Odd,
            Self::Odd => Self::Even,
        }
    }

    /// Flips the parity of `self` in place.
    pub fn flip_mut(&mut self) {
        *self = self.flip();
    }
}

/// Represents a permutation. This is stored as some storage of `usize` whose
/// entries must be numbers from 0 to the length of the permutation minus 1,
/// all pairwise different.
#[repr(transparent)]
pub struct Permutation<S: Storage<Inner = usize>>(S);

/// A statically-sized permutation, backed by an [`ArrayStorage`].
pub type PermutationS<const N: usize> = Permutation<ArrayStorageE<usize, N>>;

/// A dynamically-sized permutation, backed by an [`VecStorage`].
pub type PermutationD = Permutation<VecStorageE<usize>>;

impl<S: OwnedStorage<Inner = usize>> FromIterator<usize> for Permutation<S> {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<S: Storage<Inner = usize>> Index<usize> for Permutation<S> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<S: StorageMut<Inner = usize>> IndexMut<usize> for Permutation<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<S: ContiguousStorage<Inner = usize>> AsRef<[usize]> for Permutation<S> {
    fn as_ref(&self) -> &[usize] {
        self.0.borrow()
    }
}

impl<S: ContiguousStorageMut<Inner = usize>> AsMut<[usize]> for Permutation<S> {
    fn as_mut(&mut self) -> &mut [usize] {
        self.0.borrow_mut()
    }
}

impl<S: Storage<Inner = usize>> Permutation<S> {
    /// Returns the length of the permutation.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the size of the permutation.
    pub fn size(&self) -> Size<S> {
        self.0.size()
    }

    /// Composes `self` with another permutation and assigns to it.
    pub fn compose_mut_rhs<A: StorageMut<Inner = usize>>(&self, p: &mut Permutation<A>) {
        // Safety: composing two permutations gives a permutation.
        for v in unsafe { p.iter_mut() } {
            *v = self[*v];
        }
    }

    /// Returns an iterator over the entries of the permutation.
    pub fn iter(&self) -> iter::Copied<Iter<S>> {
        self.0.iter().copied()
    }
}

impl<S: StorageMut<Inner = usize>> Permutation<S> {
    /// Swaps two values in the permutation.
    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    /// Returns a mutable iterator over the entries of the permutation.
    ///
    /// # Safety
    /// After the iterator is called, you must ensure that what remains is a
    /// valid permutation.
    pub unsafe fn iter_mut(&mut self) -> IterMut<S> {
        self.0.iter_mut()
    }
}

impl<S: OwnedStorage<Inner = usize>> Permutation<S> {
    /// Returns the identity permutation.
    pub fn identity(size: Size<S>) -> Self {
        (0..size.value()).collect()
    }

    /// Composes two permutations together.
    pub fn compose<A: Storage<Inner = usize>, B: Storage<Inner = usize>>(
        p: &Permutation<A>,
        q: &Permutation<B>,
    ) -> Self {
        (0..p.len()).into_iter().map(|idx| p[q[idx]]).collect()
    }

    /// Returns the parity of a permutation.
    pub fn parity(&self) -> Parity {
        let mut checked = vec![false; self.len()];
        let mut parity = Parity::Even;

        for i in 0..self.len() {
            if !checked[i] {
                checked[i] = true;

                let mut j = self[i];
                let mut len = 1;
                while j != i {
                    checked[j] = true;
                    j = self[j];
                    len += 1;
                }

                if len % 2 == 1 {
                    parity.flip_mut();
                }
            }
        }

        parity
    }
}
