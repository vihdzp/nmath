use std::{
    iter::{self, FromIterator},
    ops::{Index, Mul},
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    /// Returns whether the permutation is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
    /// Initializes a new permutation. No invariants are checked.
    ///
    /// # Safety
    /// All entries of the permutation must be pairwise distinct integers, all
    /// less than the length of the permutation.
    pub unsafe fn new_unchecked(s: S) -> Self {
        Self(s)
    }

    /// Initializes a new permutation. Will check that the entries are valid. To
    /// forgo this expensive check, use [`new_unchecked`].
    pub fn new(s: S) -> Option<Self> {
        // We check that there are no repeat entries, nor entries out of bounds.
        let mut checked = vec![false; s.len()];
        for &v in s.iter() {
            let entry = checked.get_mut(v)?;

            if *entry {
                return None;
            }

            *entry = true;
        }

        // Safety: we literally just checked the invariants!
        unsafe { Some(Self::new_unchecked(s)) }
    }

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

                if len % 2 == 0 {
                    parity.flip_mut();
                }
            }
        }

        parity
    }
}

impl<S: OwnedStorage<Inner = usize>> Mul for Permutation<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<'a, S: OwnedStorage<Inner = usize>> Mul<&'a Self> for Permutation<S> {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        &self * rhs
    }
}

impl<'a, S: OwnedStorage<Inner = usize>> Mul<Permutation<S>> for &'a Permutation<S> {
    type Output = Permutation<S>;

    fn mul(self, rhs: Permutation<S>) -> Self::Output {
        self * &rhs
    }
}

impl<'a, 'b, S: OwnedStorage<Inner = usize>> Mul<&'b Permutation<S>> for &'a Permutation<S> {
    type Output = Permutation<S>;

    fn mul(self, rhs: &'b Permutation<S>) -> Self::Output {
        Permutation::compose(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose() {
        let p = PermutationS::new([0, 2, 1, 3].into()).unwrap();
        assert_eq!(p.parity(), Parity::Odd);
        assert_eq!(p * &p, PermutationS::identity(Default::default()));
    }
}
