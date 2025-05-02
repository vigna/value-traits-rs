pub trait SliceByValue<I> {
    type Value;

    fn index_value(&self, index: I) -> Self::Value;

    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_value_unchecked(&self, index: I) -> Self::Value;

    fn get_value(&self, index: I) -> Option<Self::Value>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SliceByValueMut<I>: SliceByValue<I> {
    /// Sets the value at the given index to the given value and
    /// returns the previous value, without doing bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueMut::set_value`].
    unsafe fn set_value_unchecked(&mut self, index: I, value: Self::Value) -> Self::Value;

    /// Sets the value at the given index to the given value and
    /// returns the previous value.
    fn set_value(&mut self, index: I, value: Self::Value) -> Self::Value;
}
