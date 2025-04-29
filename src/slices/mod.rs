use core::ops::Range;

pub trait IndexableBy<Idx>: SliceByValue {
    type IndexedItem;
}
impl<Idx, T> IndexableBy<Idx> for T
where
    Idx: IndexSliceByValue<T>,
    T: SliceByValue,
{
    type IndexedItem = Idx::Item;
}

pub trait IndexSliceByValue<S: SliceByValue + ?Sized> {
    type Item;
    fn get_value(&self, slice: &S) -> Option<Self::Item>;
    fn index_value(&self, slice: &S) -> Self::Item;
    unsafe fn get_value_unchecked(&self, slice: &S) -> Self::Item;
}

pub trait IndexSliceByValueMut<S: SliceByValueMut + ?Sized>: IndexSliceByValue<S> {
    fn set_value(&self, slice: &mut S, value: Self::Item) -> Self::Item;
    unsafe fn set_value_unchecked(&self, slice: &mut S, value: Self::Item) -> Self::Item;
}

pub trait Length {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SliceByValue: Length {
    /// like get_value(idx).unwrap()
    fn index_value<Idx: IndexSliceByValue<Self>>(&self, index: Idx) -> Idx::Item {
        index.index_value(self)
    }

    unsafe fn get_value_unchecked<Idx: IndexSliceByValue<Self>>(&self, index: Idx) -> Idx::Item {
        unsafe { index.get_value_unchecked(self) }
    }

    fn get_value<Idx: IndexSliceByValue<Self>>(&self, index: Idx) -> Option<Idx::Item> {
        index.get_value(self)
    }
}

struct CSR<DCF, DST>(DCF, DST)
where
    DCF: IndexableBy<usize, IndexedItem = usize>;

pub trait SliceByValueMut: SliceByValue {
    unsafe fn set_value_unchecked<Idx: IndexSliceByValueMut<Self>>(
        &self,
        index: Idx,
        value: Idx::Item,
    ) -> Idx::Item;

    fn set_value<Idx: IndexSliceByValueMut<Self>>(
        &mut self,
        index: Idx,
        value: Idx::Item,
    ) -> Idx::Item {
        index.set_value(self, value)
    }
}

impl<'a, T> SliceByValue<T> for &'a [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}
impl<'a, T> IndexSliceByValue<&'a [T]> for usize {
    type Item = T;
}

impl<'a, T> IndexSliceByValue<&'a [T]> for Range<usize> {
    type Item = &'a [T];
}
