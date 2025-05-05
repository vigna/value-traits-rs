pub trait LengthValue {
    type Value;
    /// See [`slice::len`].
    fn len(&self) -> usize;

    /// See [`slice::is_empty`].
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<S: LengthValue + ?Sized> LengthValue for &S {
    type Value = S::Value;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<S: LengthValue + ?Sized> LengthValue for &mut S {
    type Value = S::Value;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
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

// Implement SliceByValue for &S by delegating to S
impl<S: SliceByValueGet + ?Sized> SliceByValueGet for &S {
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: usize) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
}

impl<S: SliceByValueGet + ?Sized> SliceByValueGet for &mut S {
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: usize) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
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

impl<S: SliceByValueSet + ?Sized> SliceByValueSet for &mut S {
    fn set_value(&mut self, index: usize, value: Self::Value) {
        (**self).set_value(index, value)
    }
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        (**self).set_value_unchecked(index, value)
    }
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

impl<S: SliceByValueRepl + ?Sized> SliceByValueRepl for &mut S {
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        (**self).replace_value(index, value)
    }
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        (**self).replace_value_unchecked(index, value)
    }
}

pub trait SBVRL<'lend, __Implicit = &'lend Self>: LengthValue {
    type SliceRange: 'lend + SliceByValueGet<Value = Self::Value>;
}

impl<'lend, T: LengthValue + SBVRL<'lend> + ?Sized> SBVRL<'lend> for &T {
    type SliceRange = <T as SBVRL<'lend>>::SliceRange;
}
impl<'lend, T: LengthValue + SBVRL<'lend> + ?Sized> SBVRL<'lend> for &mut T {
    type SliceRange = <T as SBVRL<'lend>>::SliceRange;
}

#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type SliceRange<'lend, T: LengthValue + SBVRL<'lend>> = <T as SBVRL<'lend>>::SliceRange;

pub trait SliceByValueRange<R>: LengthValue + for<'a> SBVRL<'a> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range(&self, range: R) -> SliceRange<'_, Self>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked(&self, range: R) -> SliceRange<'_, Self>;

    /// See [`slice::get`].
    fn get_range(&self, range: R) -> Option<SliceRange<'_, Self>>;
}

impl<S: SliceByValueRange<R> + ?Sized, R> SliceByValueRange<R> for &S {
    fn get_range(&self, range: R) -> Option<SliceRange<'_, Self>> {
        (**self).get_range(range)
    }
    fn index_range(&self, range: R) -> SliceRange<'_, Self> {
        (**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&self, range: R) -> SliceRange<'_, Self> {
        (**self).get_range_unchecked(range)
    }
}
impl<S: SliceByValueRange<R> + ?Sized, R> SliceByValueRange<R> for &mut S {
    fn get_range(&self, range: R) -> Option<SliceRange<'_, Self>> {
        (**self).get_range(range)
    }
    fn index_range(&self, range: R) -> SliceRange<'_, Self> {
        (**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&self, range: R) -> SliceRange<'_, Self> {
        (**self).get_range_unchecked(range)
    }
}

pub trait SBVRML<'lend, __Implicit = &'lend Self>: LengthValue {
    type SliceRangeMut: 'lend
        + SliceByValueSet<Value = Self::Value>
        + SliceByValueRepl<Value = Self::Value>;
}

impl<'lend, T: LengthValue + SBVRML<'lend> + ?Sized> SBVRML<'lend> for &mut T {
    type SliceRangeMut = <T as SBVRML<'lend>>::SliceRangeMut;
}

#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type SliceRangeMut<'lend, T: LengthValue + SBVRML<'lend>> = <T as SBVRML<'lend>>::SliceRangeMut;

pub trait SliceByValueRangeMut<R>: LengthValue + for<'a> SBVRML<'a> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range_mut(&mut self, range: R) -> SliceRangeMut<'_, Self>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> SliceRangeMut<'_, Self>;

    /// See [`slice::get`].
    fn get_range_mut(&mut self, range: R) -> Option<SliceRangeMut<'_, Self>>;
}

impl<S: SliceByValueRangeMut<R> + ?Sized, R> SliceByValueRangeMut<R> for &mut S {
    fn get_range_mut(&mut self, range: R) -> Option<SliceRangeMut<'_, Self>> {
        (**self).get_range_mut(range)
    }
    fn index_range_mut(&mut self, range: R) -> SliceRangeMut<'_, Self> {
        (**self).index_range_mut(range)
    }
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> SliceRangeMut<'_, Self> {
        (**self).get_range_unchecked_mut(range)
    }
}
