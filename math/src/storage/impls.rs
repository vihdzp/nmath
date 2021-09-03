//! Implements the storage traits and declares the various types related to them.

use std::{
    array,
    borrow::{Borrow, BorrowMut},
    iter::{self, FromIterator},
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice, vec,
};

use super::*;

/// A convenience wrapper around the size of some storage.
///
/// By creating this value, you're asserting that you're able to create a
/// storage of this size. This isn't a problem if the storage is dynamically
/// sized. For statically sized storages, you can use the `Default`
/// implementation to initialize the size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size<S: Storage + ?Sized> {
    /// The numeric value for the size.
    value: usize,

    /// Phantom data.
    _phantom: PhantomData<S>,
}

impl<S: Storage> From<Size<S>> for usize {
    fn from(s: Size<S>) -> Self {
        s.value
    }
}

impl<S: Storage + ?Sized> Size<S> {
    /// Creates a size from a given numeric value. This method will check that
    /// this value is actually compatible with the storage.
    pub fn from_usize(value: usize) -> Self {
        if let Some(size) = S::SIZE {
            if size != value {
                panic!("size mismatch")
            }
        }

        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Returns the numeric value of the size.
    pub fn value(self) -> usize {
        self.value
    }
}

impl<S: StackStorage> Default for Size<S> {
    fn default() -> Self {
        Self::from_usize(S::SIZE_U)
    }
}

/// An iterator over the entries of a storage.
pub struct Iter<'a, S: Storage + ?Sized> {
    /// The storage.
    storage: &'a S,

    /// The index we're currently searching.
    index: usize,
}

impl<'a, S: Storage + ?Sized> Iter<'a, S> {
    pub fn new(storage: &'a S) -> Self {
        Self { storage, index: 0 }
    }
}

impl<'a, S: Storage + ?Sized> Iterator for Iter<'a, S> {
    type Item = &'a S::Inner;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.storage.get(self.index);
        self.index += 1;
        nxt
    }
}

/// A mutable iterator over the entries of a storage.
pub struct IterMut<'a, S: StorageMut + ?Sized> {
    /// The storage.
    storage: &'a mut S,

    /// The index we're currently searching.
    index: usize,
}

impl<'a, S: StorageMut + ?Sized> IterMut<'a, S> {
    pub fn new(storage: &'a mut S) -> Self {
        Self { storage, index: 0 }
    }
}

impl<'a, S: StorageMut + ?Sized> Iterator for IterMut<'a, S> {
    type Item = &'a mut S::Inner;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.storage.get_mut(self.index);
        self.index += 1;

        // Safety: the data is valid for this larger lifetime. Also, since two
        // different indices yield different mutable references, we won't cause
        // any mutable aliasing.
        unsafe { nxt.map(|data| &mut *(data as *mut _)) }
    }
}

/// A storage containing a single value. This is the building block for any
/// other storage.
///
/// The reason this exists is that otherwise, storages wouldn't have an
/// unambiguous base type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry<T>(pub T);

impl<T> Index<usize> for Entry<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self._index(index)
    }
}

impl<T> IndexMut<usize> for Entry<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._index_mut(index)
    }
}

impl<T> AsRef<T> for Entry<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Entry<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for Entry<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> Borrow<[T]> for Entry<T> {
    fn borrow(&self) -> &[T] {
        self._borrow()
    }
}

impl<T> BorrowMut<[T]> for Entry<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self._borrow_mut()
    }
}

impl<T> FromIterator<T> for Entry<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().next().unwrap())
    }
}

impl<T> IntoIterator for Entry<T> {
    type Item = T;
    type IntoIter = iter::Once<T>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.0)
    }
}

impl<T> Storage for Entry<T> {
    type Inner = T;
    const SIZE: Option<usize> = Some(1);

    fn get(&self, index: usize) -> Option<&Self::Inner> {
        (index == 0).then(|| &self.0)
    }

    fn len(&self) -> usize {
        1
    }
}

unsafe impl<T> StorageMut for Entry<T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Inner> {
        (index == 0).then(move || &mut self.0)
    }
}

unsafe impl<T> StackStorage for Entry<T> {}

/// Stores a reference to a storage. This will also be a storage.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ref<'a, S: Storage>(pub &'a S);

impl<'a, S: Storage> Index<usize> for Ref<'a, S> {
    type Output = S::Inner;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, S: Storage> Storage for Ref<'a, S> {
    type Inner = S::Inner;
    const SIZE: Option<usize> = S::SIZE;

    fn get(&self, index: usize) -> Option<&Self::Inner> {
        self.0.get(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, S: ContiguousStorage> Borrow<[S::Inner]> for Ref<'a, S> {
    fn borrow(&self) -> &[S::Inner] {
        self.0.borrow()
    }
}

/// Stores a mutable reference to a storage. This will also be a storage.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mut<'a, S: Storage>(pub &'a mut S);

impl<'a, S: Storage> Index<usize> for Mut<'a, S> {
    type Output = S::Inner;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, S: StorageMut> IndexMut<usize> for Mut<'a, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a, S: Storage> Storage for Mut<'a, S> {
    type Inner = S::Inner;
    const SIZE: Option<usize> = S::SIZE;

    fn get(&self, index: usize) -> Option<&Self::Inner> {
        self.0.get(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

unsafe impl<'a, S: StorageMut> StorageMut for Mut<'a, S> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Inner> {
        self.0.get_mut(index)
    }
}

impl<'a, S: ContiguousStorage> Borrow<[S::Inner]> for Mut<'a, S> {
    fn borrow(&self) -> &[S::Inner] {
        (&*self.0).borrow()
    }
}

impl<'a, S: ContiguousStorageMut> BorrowMut<[S::Inner]> for Mut<'a, S> {
    fn borrow_mut(&mut self) -> &mut [S::Inner] {
        self.0.borrow_mut()
    }
}

/// An array of a single storage type. This will have the same layout as an
/// array of the inner type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayStorage<S: StackStorage, const N: usize>(pub [S; N]);

/// An array storage whose entries are of a given type.
pub type ArrayStorageE<T, const N: usize> = ArrayStorage<Entry<T>, N>;

impl<S: StackStorage, const N: usize> Default for ArrayStorage<S, N>
where
    S::Inner: Default + Clone,
{
    fn default() -> Self {
        iter::repeat(Default::default()).collect()
    }
}

impl<S: StackStorage, const N: usize> Borrow<[S::Inner]> for ArrayStorage<S, N> {
    fn borrow(&self) -> &[S::Inner] {
        self._borrow()
    }
}

impl<S: StackStorage, const N: usize> BorrowMut<[S::Inner]> for ArrayStorage<S, N> {
    fn borrow_mut(&mut self) -> &mut [S::Inner] {
        self._borrow_mut()
    }
}

impl<S: StackStorage, const N: usize> FromIterator<S::Inner> for ArrayStorage<S, N> {
    fn from_iter<T: IntoIterator<Item = S::Inner>>(iter: T) -> Self {
        Self::_from_iter(iter)
    }
}

/// The type of the `into_iter` function associated to a certain iterator.
type IntoIterFn<T> = fn(T) -> <T as IntoIterator>::IntoIter;

/// The type of a flattened iterator over inner iterators.
type FlatIntoIter<I, S> = iter::Flatten<iter::Map<I, IntoIterFn<S>>>;

impl<S: StackStorage, const N: usize> IntoIterator for ArrayStorage<S, N> {
    type Item = S::Inner;
    type IntoIter = FlatIntoIter<array::IntoIter<S, N>, S>;

    fn into_iter(self) -> Self::IntoIter {
        array::IntoIter::new(self.0)
            .map(IntoIterator::into_iter as IntoIterFn<S>)
            .flatten()
    }
}

impl<S: StackStorage, const N: usize> Index<usize> for ArrayStorage<S, N> {
    type Output = S::Inner;

    fn index(&self, index: usize) -> &Self::Output {
        self._index(index)
    }
}

impl<S: StackStorage, const N: usize> IndexMut<usize> for ArrayStorage<S, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._index_mut(index)
    }
}

const fn times(size: Option<usize>, n: usize) -> Option<usize> {
    if let Some(u) = size {
        Some(u * n)
    } else {
        None
    }
}

impl<S: StackStorage, const N: usize> Storage for ArrayStorage<S, N> {
    type Inner = S::Inner;

    const SIZE: Option<usize> = times(S::SIZE, N);

    fn get(&self, index: usize) -> Option<&Self::Inner> {
        self._get(index)
    }

    fn len(&self) -> usize {
        N
    }
}

unsafe impl<S: StackStorage, const N: usize> StorageMut for ArrayStorage<S, N> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Inner> {
        self._get_mut(index)
    }
}

unsafe impl<S: StackStorage, const N: usize> StackStorage for ArrayStorage<S, N> {}

/// An auxiliary struct that joins two buffers together.
///
/// This type is `#[repr(C)]`, so initializing it with two stack storages of
/// type `T` will give another one.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Join<A: StackStorage, B: StackStorage>(A, B);

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> Join<A, B> {
    /// Initializes a new joined buffer.
    pub fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> Index<usize> for Join<A, B> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self._index(index)
    }
}

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> IndexMut<usize> for Join<A, B> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._index_mut(index)
    }
}

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> Borrow<[T]> for Join<A, B> {
    fn borrow(&self) -> &[T] {
        self._borrow()
    }
}

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> BorrowMut<[T]> for Join<A, B> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self._borrow_mut()
    }
}

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> FromIterator<T> for Join<A, B> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::_from_iter(iter)
    }
}

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> IntoIterator for Join<A, B> {
    type Item = T;
    type IntoIter = iter::Chain<A::IntoIter, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().chain(self.1.into_iter())
    }
}

const fn add(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    if let (Some(a), Some(b)) = (a, b) {
        Some(a + b)
    } else {
        None
    }
}

impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> Storage for Join<A, B> {
    type Inner = T;

    const SIZE: Option<usize> = add(A::SIZE, B::SIZE);

    fn get(&self, index: usize) -> Option<&Self::Inner> {
        self._get(index)
    }

    fn len(&self) -> usize {
        Self::SIZE_U
    }
}

unsafe impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> StorageMut for Join<A, B> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Inner> {
        self._get_mut(index)
    }
}

unsafe impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> StackStorage for Join<A, B> {}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// A heap-allocated storage. Is backed by a `Vec`.
pub struct VecStorage<S: StackStorage>(pub Vec<S>);

/// A vec storage whose entries are of a given type.
pub type VecStorageE<T> = VecStorage<Entry<T>>;

impl<S: StackStorage> Default for VecStorage<S> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<S: StackStorage> VecStorage<S> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<S: StackStorage> Index<usize> for VecStorage<S> {
    type Output = S::Inner;

    fn index(&self, index: usize) -> &Self::Output {
        self._index(index)
    }
}

impl<S: StackStorage> IndexMut<usize> for VecStorage<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._index_mut(index)
    }
}

impl<S: StackStorage> Borrow<[S::Inner]> for VecStorage<S> {
    fn borrow(&self) -> &[S::Inner] {
        // Safety: the inner storage is made out of various stack storages,
        // which have the same alignment of the inner type. Therefore, the read
        // is valid and aligned.
        unsafe { slice::from_raw_parts(self.0.as_ptr() as *const _, self.0.len() * S::SIZE_U) }
    }
}

impl<S: StackStorage> BorrowMut<[S::Inner]> for VecStorage<S> {
    fn borrow_mut(&mut self) -> &mut [S::Inner] {
        // Safety: the inner storage is made out of various stack storages,
        // which have the same alignment of the inner type. Therefore, the write
        // is valid and aligned.
        unsafe {
            slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut _, self.0.len() * S::SIZE_U)
        }
    }
}

impl<S: StackStorage> FromIterator<S::Inner> for VecStorage<S> {
    fn from_iter<I: IntoIterator<Item = S::Inner>>(iter: I) -> Self {
        if S::SIZE_U == 0 {
            return Self(Vec::new());
        }

        let mut iter = iter.into_iter();
        let mut vec = Vec::with_capacity(iter.size_hint().0);

        while let Some(val) = iter.next() {
            let mut uninit = UninitStackStorage::new();
            uninit.push(val);

            for _ in 1..S::SIZE_U {
                uninit.push(iter.next().expect("iterator could not fill inner type"))
            }

            vec.push(uninit.init());
        }

        Self(vec)
    }
}

impl<S: StackStorage> IntoIterator for VecStorage<S> {
    type Item = S::Inner;
    type IntoIter = FlatIntoIter<vec::IntoIter<S>, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(IntoIterator::into_iter as IntoIterFn<S>)
            .flatten()
    }
}

impl<S: StackStorage> Storage for VecStorage<S> {
    type Inner = S::Inner;

    const SIZE: Option<usize> = None;

    fn get(&self, index: usize) -> Option<&Self::Inner> {
        self._get(index)
    }

    fn len(&self) -> usize {
        self._len()
    }
}

unsafe impl<S: StackStorage> StorageMut for VecStorage<S> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Inner> {
        self._get_mut(index)
    }
}

/// A trait that specifies that a storage may be creating by concatenating
/// together two other buffers.
pub trait JoinStorage<A: Storage, B: Storage>: Storage {
    /// Merges two buffers together.
    fn merge(a: A, b: B) -> Self;
}

/// Any two stack buffers may be merged into a `Join`.
impl<T, A: StackStorage<Inner = T>, B: StackStorage<Inner = T>> JoinStorage<A, B> for Join<A, B> {
    fn merge(a: A, b: B) -> Self {
        Self::new(a, b)
    }
}

/// Two owned buffers may be joined into a `VecStorage`.
impl<T: StackStorage, A: OwnedStorage<Inner = T>, B: OwnedStorage<Inner = T>> JoinStorage<A, B>
    for VecStorage<T>
{
    fn merge(a: A, b: B) -> Self {
        let mut vec = VecStorage::with_capacity(a.len() + b.len());
        vec.0.extend(a.into_iter());
        vec.0.extend(b.into_iter());
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge() {
        let a = ArrayStorage([ArrayStorage([Entry(Box::new(1)), Entry(Box::new(2))])]);
        let b = ArrayStorage([Entry(Box::new(3))]);
        let c = ArrayStorage([Entry(Box::new(4))]);
        let d = a.merge_join(b).merge_join(c).transmute_array::<4>();

        assert_eq!(
            d,
            ArrayStorage([
                Entry(Box::new(1)),
                Entry(Box::new(2)),
                Entry(Box::new(3)),
                Entry(Box::new(4))
            ])
        );
    }
}
