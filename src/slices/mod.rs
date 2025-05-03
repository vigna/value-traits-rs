pub trait Length {
    /// See [`slice::len`].
    fn len(&self) -> usize;

    /// See [`slice::is_empty`].
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SliceByValueGet<I>: Length {
    type Value;
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_value(&self, index: I) -> Self::Value;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_value_unchecked(&self, index: I) -> Self::Value;

    /// See [`slice::get`].
    fn get_value(&self, index: I) -> Option<Self::Value>;
}

pub trait SliceByValueSet<I>: Length {
    type Value;
    /// Sets the value at the given index to the given value without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueMut::set_value`].
    unsafe fn set_value_unchecked(&mut self, index: I, value: Self::Value);

    /// Sets the value at the given index to the given value.
    fn set_value(&mut self, index: I, value: Self::Value);
}

pub trait SliceByValueRepl<I>: Length {
    type Value;
    /// Sets the value at the given index to the given value and
    /// returns the previous value, without doing bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueMut::set_value`].
    unsafe fn replace_value_unchecked(&mut self, index: I, value: Self::Value) -> Self::Value;

    /// Sets the value at the given index to the given value and
    /// returns the previous value.
    fn replace_value(&mut self, index: I, value: Self::Value) -> Self::Value;
}
