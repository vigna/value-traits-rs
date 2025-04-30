use slices::{SliceByValue, SliceByValueIndex};

pub mod iter;
pub mod slices;

impl<'a, T> SliceByValue for &'a [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T: Clone> SliceByValueIndex<&[T]> for usize {
    type Value = T;

    fn get_value(&self, slice: &&[T]) -> Option<Self::Value> {
        slice.get(*self).map(|v| v.clone())
    }
    fn index_value(&self, slice: &&[T]) -> Self::Value {
        slice[*self].clone()
    }
    unsafe fn get_value_unchecked(&self, slice: &&[T]) -> Self::Value {
        slice.get_unchecked(*self).clone()
    }
}
