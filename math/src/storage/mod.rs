//! Declares the different storage types and traits.

pub mod impls;
pub use impls::*;

use core::slice;
use std::{
    borrow::{Borrow, BorrowMut},
    iter::FromIterator,
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::{Index, IndexMut},
    ptr,
};

/// The most general storage trait. We merely require that we can index into the
/// data, and that the storage has some length. Getting a value with an index
/// less than the length must always be valid.
///
/// Types implementing this trait need not own the data or even have mutable
/// access to it.
pub trait Storage: Index<usize, Output = Self::Inner> {
    /// The inner type that's being stored.
    type Inner;

    /// The size of the storage if statically known, or `None` otherwise.
    const SIZE: Option<usize>;

    /// Gets a reference to an element of a given index. This must produce a
    /// `Some` whenever `index < self.len()`.
    fn get(&self, index: usize) -> Option<&Self::Inner>;

    /// Returns the length of the storage.
    fn len(&self) -> usize;

    /// Returns the size of the storage.
    fn size(&self) -> Size<Self> {
        Size::from_usize(self.len())
    }

    /// Returns an iterator over references of the storage.
    fn iter(&self) -> Iter<'_, Self> {
        Iter::new(self)
    }
}

/// A [`Storage`] that has mutable access to its data.
///
/// # Safety
/// Different elements must yield different references, or mutable iterators
/// over this storage will break the aliasing rule.
pub unsafe trait StorageMut: Storage + IndexMut<usize, Output = Self::Inner> {
    /// Gets a mutable reference to an element of a given index. This must
    /// produce a `Some` whenever `index < self.len()`.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Inner>;

    /// Swaps the values at two indices.
    fn swap(&mut self, i: usize, j: usize) {
        let a = self.get_mut(i).unwrap() as *mut _;
        let b = self.get_mut(j).unwrap() as *mut _;

        unsafe {
            ptr::swap(a, b);
        }
    }

    /// Returns an iterator over mutable references of the storage.
    fn iter_mut(&mut self) -> IterMut<'_, Self> {
        IterMut::new(self)
    }
}

/// A [`Storage`] whose data is stored contiguously in memory. That is, we're
/// able to provide a slice to the storage.
pub trait ContiguousStorage: Storage + Borrow<[Self::Inner]> {
    /// Returns the inner slice storage.
    fn as_slice(&self) -> &[Self::Inner] {
        self.borrow()
    }

    /// An automatically-generated implementation of the [`get`] method.
    fn _get(&self, index: usize) -> Option<&Self::Inner> {
        self.as_slice().get(index)
    }

    /// An automatically-generated implementation of the [`len`] method.
    fn _len(&self) -> usize {
        self.as_slice().len()
    }

    /// An automatically-generated implementation of the [`Index`] trait.
    fn _index(&self, index: usize) -> &Self::Inner {
        &self.as_slice()[index]
    }
}

impl<T: Storage + Borrow<[Self::Inner]>> ContiguousStorage for T {}

/// A [`StorageMut`] whose data is stored contiguously in memory. That is, we're
/// able to provide a mutable slice to the storage.
pub trait ContiguousStorageMut: StorageMut + ContiguousStorage + BorrowMut<[Self::Inner]> {
    /// Returns the inner mutable slice storage.
    fn as_mut_slice(&mut self) -> &mut [Self::Inner] {
        self.borrow_mut()
    }

    /// An automatically-generated implementation of the [`get_mut`] method.   
    fn _get_mut(&mut self, index: usize) -> Option<&mut Self::Inner> {
        self.as_mut_slice().get_mut(index)
    }

    /// An automatically-generated implementation of the [`IndexMut`] trait.
    fn _index_mut(&mut self, index: usize) -> &mut Self::Inner {
        &mut self.as_mut_slice()[index]
    }
}

impl<T: StorageMut + BorrowMut<[Self::Inner]>> ContiguousStorageMut for T {}



/// A storage that owns its data. We certify that this is the case by requiring
/// the implementation of [`FromIterator`] and [`IntoIterator`].
///
/// An owned storage can either have a fixed static size, or it must be
/// initializable with any size, depending on whether `SIZE` is `Some` or `None`.
pub trait OwnedStorage:
    StorageMut + FromIterator<Self::Inner> + IntoIterator<Item = Self::Inner>
{
}

impl<T: StorageMut + FromIterator<Self::Inner> + IntoIterator<Item = Self::Inner>> OwnedStorage
    for T
{
}

/// A wrapper around a stack-allocated storage, which allows one to set the
/// elements from left to right.
pub struct UninitStackStorage<S: StackStorage> {
    /// The storage.
    storage: MaybeUninit<S>,

    /// The amount of elements we've set.
    len: usize,
}

impl<S: StackStorage> UninitStackStorage<S> {
    /// Builds a new uninitialized stack storage.
    pub fn new() -> Self {
        Self {
            storage: MaybeUninit::uninit(),
            len: 0,
        }
    }

    /// Returns a pointer to the inner storage.
    fn ptr_mut(&mut self) -> *mut S::Inner {
        &mut self.storage as *mut _ as *mut _
    }

    /// Pushes a value into the storage.
    ///
    /// # Panics
    /// This method will panic if more than `S::SIZE` elements are added.
    pub fn push(&mut self, value: S::Inner) {
        if self.len < S::SIZE_U {
            // Safety: this must be in bounds.
            unsafe {
                self.ptr_mut().offset(self.len as isize).write(value);
            }
        } else {
            panic!("stack storage full")
        }

        self.len += 1;
    }

    /// Returns the initialized storage.
    ///
    /// # Panics
    /// This method will panic if the storage hasn't been fully initialized.
    pub fn init(self) -> S {
        if self.len == S::SIZE_U {
            // Safety: this storage is initialized, and we avoid double-drops.
            unsafe { mem::transmute_copy(&ManuallyDrop::new(self).storage) }
        } else {
            panic!("stack storage not full")
        }
    }
}

impl<S: StackStorage> Extend<S::Inner> for UninitStackStorage<S> {
    fn extend<T: IntoIterator<Item = S::Inner>>(&mut self, iter: T) {
        for v in iter {
            self.push(v);
        }
    }
}

impl<S: StackStorage> FromIterator<S::Inner> for UninitStackStorage<S> {
    fn from_iter<T: IntoIterator<Item = S::Inner>>(iter: T) -> Self {
        let mut storage = Self::new();
        storage.extend(iter);
        storage
    }
}

impl<S: StackStorage> Drop for UninitStackStorage<S> {
    fn drop(&mut self) {
        let ptr = self.ptr_mut();

        for i in 0..self.len as isize {
            // Safety: all of these writes are in bounds. Also, since our
            // storage was `MaybeUninit`, we avoid double-dropping.
            unsafe {
                ManuallyDrop::drop(&mut *(ptr.offset(i) as *mut ManuallyDrop<S::Inner>));
            }
        }
    }
}

const fn unwrap(size: Option<usize>) -> usize {
    if let Some(u) = size {
        u
    } else {
        usize::MAX
    }
}

/// A stack-allocated contiguous data buffer. The size of the buffer must be
/// statically known.
///
/// Contrary to [`ContiguousStorage`], the storage cannot contain any fields
/// other than the data itself. This is so that we can guarantee the layout of
/// structs like [`ArrayStorage`] and [`Join`].
///
/// # Safety
/// We require that the type has the same layout and alignment as
/// `[T; Self::SIZE]`.
pub unsafe trait StackStorage: ContiguousStorageMut + OwnedStorage {
    /// The size of the buffer.
    const SIZE_U: usize = unwrap(Self::SIZE);

    /// An automatic implementation of the `FromIterator` trait.
    fn _from_iter<I: IntoIterator<Item = Self::Inner>>(iter: I) -> Self {
        iter.into_iter().collect::<UninitStackStorage<_>>().init()
    }

    /// An automatic implementation of the `Borrow<[Self::Inner]>` trait.
    fn _borrow(&self) -> &[Self::Inner] {
        unsafe { slice::from_raw_parts(self as *const _ as *const _, Self::SIZE_U) }
    }

    /// An automatic implementation of the `BorrowMut<[Self::Inner]>` trait.
    fn _borrow_mut(&mut self) -> &mut [Self::Inner] {
        unsafe { slice::from_raw_parts_mut(self as *mut _ as *mut _, Self::SIZE_U) }
    }

    /// Merges two stack storages into a joined type.
    fn merge_join<A: StackStorage<Inner = Self::Inner>>(self, other: A) -> Join<Self, A> {
        Join::new(self, other)
    }

    /// Transmutes a stack buffer into another of the same size. This operation
    /// should be zero-cost unless you expose pointers to both the input and
    /// output.
    ///
    /// Despite the name, this operation is **perfectly safe**, assuming that
    /// [`StackBuffer`] has been properly implemented for both types.
    ///
    /// # Panics
    /// This operation will panic if `Self::SIZE != U::SIZE`. The user must
    /// certify this condition themselves.
    fn transmute_buffer<S: StackStorage<Inner = Self::Inner>>(self) -> S {
        // The important assertion.
        assert_eq!(Self::SIZE, S::SIZE);

        // For good measure.
        assert_eq!(mem::size_of::<Self>(), mem::size_of::<S>());
        assert_eq!(mem::align_of::<Self>(), mem::align_of::<S>());

        // Safety: Any two stack buffers of the same size should have the same
        // layout. Also, there's no double-drops.
        unsafe { mem::transmute_copy(&ManuallyDrop::new(self)) }
    }

    /// Transmutes a stack buffer into an array of the same size. Due to
    /// limitations on const-generics, you must specify this size yourself. This
    /// operation should be zero-cost unless you expose pointers to both the
    /// input and output.
    ///
    /// Despite the name, this operation is **perfectly safe**, assuming that
    /// [`StackBuffer`] has been properly implemented for this type.
    ///
    /// # Panics
    /// This operation will panic if `Self::SIZE != N`. The user must certify
    /// this condition themselves.
    fn transmute_array<const N: usize>(self) -> ArrayStorage<Entry<Self::Inner>, N> {
        self.transmute_buffer()
    }
}
