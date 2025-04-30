use core::ops::Range;

pub trait SliceByValue {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_value<I: SliceByValueIndex<Self>>(&self, index: I) -> I::Output {
        index.index_value(self)
    }

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_value_unchecked<I: SliceByValueIndex<Self>>(&self, index: I) -> I::Output {
        unsafe { index.get_value_unchecked(self) }
    }

    /// See [`slice::get`].
    fn get_value<I: SliceByValueIndex<Self>>(&self, index: I) -> Option<I::Output> {
        index.get_value(self)
    }

    /// See [`slice::len`].
    fn len(&self) -> usize;

    /// See [`slice::is_empty`].
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SliceByValueMut: SliceByValue {
    /// Sets the value at the given index to the given value and
    /// returns the previous value, without doing bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueMut::set_value`].
    unsafe fn set_value_unchecked<I: SliceByValueMutIndex<Self>>(
        &mut self,
        index: I,
        value: I::Output,
    ) -> I::Output {
        unsafe { index.set_value_unchecked(self, value) }
    }

    /// Sets the value at the given index to the given value and
    /// returns the previous value.
    fn set_value<I: SliceByValueMutIndex<Self>>(
        &mut self,
        index: I,
        value: I::Output,
    ) -> I::Output {
        index.set_value(self, value)
    }
}

pub trait SliceByValueIndex<S: SliceByValue + ?Sized> {
    type Output;
    fn get_value(&self, slice: &S) -> Option<Self::Output>;
    fn index_value(&self, slice: &S) -> Self::Output;
    unsafe fn get_value_unchecked(&self, slice: &S) -> Self::Output;
}

pub trait SliceByValueMutIndex<S: SliceByValueMut + ?Sized>: SliceByValueIndex<S> {
    fn set_value(&self, slice: &mut S, value: Self::Output) -> Self::Output;
    unsafe fn set_value_unchecked(&self, slice: &mut S, value: Self::Output) -> Self::Output;
}

/// Convenience trait for specifying the behavior of a
/// by-value slice.
///
/// This traits makes it possible to write trait bounds as
/// ```ignore
/// T: IndexableBy<usize, Output = int32>
/// ```
/// instead of the equivalent `where` clause
/// ```ignore
/// where T: SliceByValue, usize: SliceByValueIndex<Output = i32>
/// ```
pub trait IndexableBy<I>: SliceByValue {
    type Output;
}

impl<I, T> IndexableBy<I> for T
where
    I: SliceByValueIndex<T>,
    T: SliceByValue,
{
    type Output = I::Output;
}

pub trait IndexableByMut<I>: SliceByValueMut {
    type Output;
}

impl<I, T> IndexableByMut<I> for T
where
    I: SliceByValueMutIndex<T>,
    T: SliceByValueMut,
{
    type Output = I::Output;
}
