#![deny(unconditional_recursion)]
use slices::{SliceByValue, SliceByValueMut};

pub mod iter;
pub mod slices;

// --- Implementations for standard slices [T] and usize index ---
impl<T: Copy> SliceByValue<usize> for [T] {
    type Value = T;

    #[inline]
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        // slice.get returns Option<&T>, .copied() converts to Option<T>
        (*self).get(index).copied()
    }

    #[inline]
    fn index_value(&self, index: usize) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // It returns &T, which we copy to return T.
        self[index]
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        // slice.get_unchecked returns &T, which we dereference and copy.
        unsafe { *(*self).get_unchecked(index) }
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T: Copy> SliceByValueMut<usize> for [T] {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // We get a mutable reference `&mut T`.
        // mem::replace swaps the value at the location with the new `value`
        // and returns the old value.
        core::mem::replace(&mut self[index], value)
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            core::mem::replace(elem, value)
        }
    }
}

// Implement SliceByValue for &S by delegating to S
impl<I, S: SliceByValue<I> + ?Sized> SliceByValue<I> for &S {
    type Value = S::Value;

    fn get_value(&self, index: I) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: I) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: I) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
    #[inline]
    fn len(&self) -> usize {
        // `self` is `&&S`, so `*self` is `&S`, `**self` is `S`
        (**self).len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

// Implement SliceByValue for &mut S by delegating to S (for read-only access)
impl<I, S: SliceByValue<I> + ?Sized> SliceByValue<I> for &mut S {
    type Value = S::Value;
    fn get_value(&self, index: I) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: I) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: I) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

impl<I, S: SliceByValueMut<I> + ?Sized> SliceByValueMut<I> for &mut S {
    fn set_value(&mut self, index: I, value: Self::Value) -> Self::Value {
        (**self).set_value(index, value)
    }
    unsafe fn set_value_unchecked(&mut self, index: I, value: Self::Value) -> Self::Value {
        (**self).set_value_unchecked(index, value)
    }
}
