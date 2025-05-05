use core::ops::Range;

pub trait LengthValue {
    type Value;
    /// See [`slice::len`].
    fn len(&self) -> usize;

    /// See [`slice::is_empty`].
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SliceByValueGet: LengthValue {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_value(&self, index: usize) -> Self::Value;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value;

    /// See [`slice::get`].
    fn get_value(&self, index: usize) -> Option<Self::Value>;
}

pub trait SliceByValueSet: LengthValue {
    /// Sets the value at the given index to the given value without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueMut::set_value`].
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value);

    /// Sets the value at the given index to the given value.
    fn set_value(&mut self, index: usize, value: Self::Value);
}

pub trait SliceByValueRepl: LengthValue {
    /// Sets the value at the given index to the given value and
    /// returns the previous value, without doing bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueMut::set_value`].
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value;

    /// Sets the value at the given index to the given value and
    /// returns the previous value.
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value;
}

pub trait SliceByValueRange<R>: LengthValue {
    type SliceRange<'a>: SliceByValueGet<Value = Self::Value>
    where
        Self: 'a;
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range(&self, range: R) -> Self::SliceRange<'_>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked(&self, range: R) -> Self::SliceRange<'_>;

    /// See [`slice::get`].
    fn get_range(&self, range: R) -> Option<Self::SliceRange<'_>>;
}

pub trait SliceByValueRangeMut<R>: LengthValue {
    type SliceRangeMut<'a>: SliceByValueSet<Value = Self::Value>
        + SliceByValueRepl<Value = Self::Value>
    where
        Self: 'a;
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range_mut(&mut self, range: R) -> Self::SliceRangeMut<'_>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> Self::SliceRangeMut<'_>;

    /// See [`slice::get`].
    fn get_range_mut(&mut self, range: R) -> Option<Self::SliceRangeMut<'_>>;
}
