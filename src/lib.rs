use slices::{SliceByValue, SliceByValueIndex, SliceByValueMut, SliceByValueMutIndex};

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

impl<'a, T> SliceByValue for &'a mut [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T: Clone> SliceByValueIndex<&mut [T]> for usize {
    type Value = T;

    fn get_value(&self, slice: &&mut [T]) -> Option<Self::Value> {
        slice.get(*self).map(|v| v.clone())
    }
    fn index_value(&self, slice: &&mut [T]) -> Self::Value {
        slice[*self].clone()
    }
    unsafe fn get_value_unchecked(&self, slice: &&mut [T]) -> Self::Value {
        slice.get_unchecked(*self).clone()
    }
}

impl<'a, T> SliceByValueMut for &'a mut [T] {}

impl<T: Clone> SliceByValueMutIndex<&mut [T]> for usize {
    // TODO: check this
    fn set_value(&self, slice: &mut &mut [T], value: Self::Value) -> Self::Value {
        let old_value = slice.get(*self).cloned();
        if let Some(v) = old_value {
            slice[*self] = value;
            v
        } else {
            panic!("Index out of bounds")
        }
    }

    unsafe fn set_value_unchecked(&self, slice: &mut &mut [T], value: Self::Value) -> Self::Value {
        let old_value = slice.get_unchecked(*self).clone();
        *slice.get_unchecked_mut(*self) = value;
        old_value
    }
}
