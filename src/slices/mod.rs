use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

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

pub trait SBVRL<'a, R, __Implicit = &'a Self>: LengthValue {
    type SliceRange: 'a
        + SliceByValueGet<Value = Self::Value>
        + SBVRL<'a, R, SliceRange = Self::SliceRange> // recursion
        + SliceByValueRange<R>;
}

impl<'a, R, T: LengthValue + SBVRL<'a, R> + ?Sized> SBVRL<'a, R> for &T {
    type SliceRange = <T as SBVRL<'a, R>>::SliceRange;
}
impl<'a, R, T: LengthValue + SBVRL<'a, R> + ?Sized> SBVRL<'a, R> for &mut T {
    type SliceRange = <T as SBVRL<'a, R>>::SliceRange;
}

#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type SliceRange<'a, R, T: LengthValue + SBVRL<'a, R>> = <T as SBVRL<'a, R>>::SliceRange;

pub trait SliceByValueRange<R>: LengthValue + for<'a> SBVRL<'a, R> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range(&self, range: R) -> SliceRange<'_, R, Self>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked(&self, range: R) -> SliceRange<'_, R, Self>;

    /// See [`slice::get`].
    fn get_range(&self, range: R) -> Option<SliceRange<'_, R, Self>>;
}

impl<S: SliceByValueRange<R> + ?Sized, R> SliceByValueRange<R> for &S {
    fn get_range(&self, range: R) -> Option<SliceRange<'_, R, Self>> {
        (**self).get_range(range)
    }
    fn index_range(&self, range: R) -> SliceRange<'_, R, Self> {
        (**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&self, range: R) -> SliceRange<'_, R, Self> {
        (**self).get_range_unchecked(range)
    }
}
impl<S: SliceByValueRange<R> + ?Sized, R> SliceByValueRange<R> for &mut S {
    fn get_range(&self, range: R) -> Option<SliceRange<'_, R, Self>> {
        (**self).get_range(range)
    }
    fn index_range(&self, range: R) -> SliceRange<'_, R, Self> {
        (**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&self, range: R) -> SliceRange<'_, R, Self> {
        (**self).get_range_unchecked(range)
    }
}

pub trait SBVRML<'a, R, __Implicit = &'a Self>: LengthValue {
    type SliceRangeMut: 'a
        + SliceByValueSet<Value = Self::Value>
        + SliceByValueRepl<Value = Self::Value>
        + SBVRML<'a, R, SliceRangeMut = Self::SliceRangeMut> // recursion
        + SliceByValueRangeMut<R>;
}

impl<'a, R, T: LengthValue + SBVRML<'a, R> + ?Sized> SBVRML<'a, R> for &mut T {
    type SliceRangeMut = <T as SBVRML<'a, R>>::SliceRangeMut;
}

#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type SliceRangeMut<'a, R, T: LengthValue + SBVRML<'a, R>> = <T as SBVRML<'a, R>>::SliceRangeMut;

pub trait SliceByValueRangeMut<R>: LengthValue + for<'a> SBVRML<'a, R> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range_mut(&mut self, range: R) -> SliceRangeMut<'_, R, Self>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> SliceRangeMut<'_, R, Self>;

    /// See [`slice::get`].
    fn get_range_mut(&mut self, range: R) -> Option<SliceRangeMut<'_, R, Self>>;
}

impl<S: SliceByValueRangeMut<R> + ?Sized, R> SliceByValueRangeMut<R> for &mut S {
    fn get_range_mut(&mut self, range: R) -> Option<SliceRangeMut<'_, R, Self>> {
        (**self).get_range_mut(range)
    }
    fn index_range_mut(&mut self, range: R) -> SliceRangeMut<'_, R, Self> {
        (**self).index_range_mut(range)
    }
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> SliceRangeMut<'_, R, Self> {
        (**self).get_range_unchecked_mut(range)
    }
}

/// Helper trait for requesting all common range types, and enforce that they all
/// return the same type of slice.
pub trait SliceByValueRangeAll<T = usize>:
    SliceByValueRange<Range<T>>
    + SliceByValueRange<RangeFrom<T>>
    + SliceByValueRange<RangeFull>
    + SliceByValueRange<RangeInclusive<T>>
    + SliceByValueRange<RangeTo<T>>
    + SliceByValueRange<RangeToInclusive<T>>
    + for<'a> SBVRL<'a, Range<T>>
    + for<'a> SBVRL<'a, RangeFrom<T>, SliceRange = <Self as SBVRL<'a, Range<T>>>::SliceRange>
    + for<'a> SBVRL<'a, RangeFull, SliceRange = <Self as SBVRL<'a, Range<T>>>::SliceRange>
    + for<'a> SBVRL<'a, RangeInclusive<T>, SliceRange = <Self as SBVRL<'a, Range<T>>>::SliceRange>
    + for<'a> SBVRL<'a, RangeTo<T>, SliceRange = <Self as SBVRL<'a, Range<T>>>::SliceRange>
    + for<'a> SBVRL<'a, RangeToInclusive<T>, SliceRange = <Self as SBVRL<'a, Range<T>>>::SliceRange>
{
}

impl<U, T> SliceByValueRangeAll<T> for U
where
    U: SliceByValueRange<Range<T>>,
    U: SliceByValueRange<RangeFrom<T>>,
    U: SliceByValueRange<RangeFull>,
    U: SliceByValueRange<RangeInclusive<T>>,
    U: SliceByValueRange<RangeTo<T>>,
    U: SliceByValueRange<RangeToInclusive<T>>,
    U: for<'a> SBVRL<'a, Range<T>>,
    U: for<'a> SBVRL<'a, RangeFrom<T>, SliceRange = <U as SBVRL<'a, Range<T>>>::SliceRange>,
    U: for<'a> SBVRL<'a, RangeFull, SliceRange = <U as SBVRL<'a, Range<T>>>::SliceRange>,
    U: for<'a> SBVRL<'a, RangeInclusive<T>, SliceRange = <U as SBVRL<'a, Range<T>>>::SliceRange>,
    U: for<'a> SBVRL<'a, RangeTo<T>, SliceRange = <U as SBVRL<'a, Range<T>>>::SliceRange>,
    U: for<'a> SBVRL<'a, RangeToInclusive<T>, SliceRange = <U as SBVRL<'a, Range<T>>>::SliceRange>,
{
}

/// Mutable version of [`SliceByValueRangeAll`].
pub trait SliceByValueRangeAllMut<T = usize>:
    SliceByValueRangeMut<Range<T>>
    + SliceByValueRangeMut<RangeFrom<T>>
    + SliceByValueRangeMut<RangeFull>
    + SliceByValueRangeMut<RangeInclusive<T>>
    + SliceByValueRangeMut<RangeTo<T>>
    + SliceByValueRangeMut<RangeToInclusive<T>>
    + for<'a> SBVRML<'a, Range<T>>
    + for<'a> SBVRML<'a, RangeFrom<T>, SliceRangeMut = <Self as SBVRML<'a, Range<T>>>::SliceRangeMut>
    + for<'a> SBVRML<'a, RangeFull, SliceRangeMut = <Self as SBVRML<'a, Range<T>>>::SliceRangeMut>
    + for<'a> SBVRML<'a, RangeInclusive<T>, SliceRangeMut = <Self as SBVRML<'a, Range<T>>>::SliceRangeMut>
    + for<'a> SBVRML<'a, RangeTo<T>, SliceRangeMut = <Self as SBVRML<'a, Range<T>>>::SliceRangeMut>
    + for<'a> SBVRML<'a, RangeToInclusive<T>, SliceRangeMut = <Self as SBVRML<'a, Range<T>>>::SliceRangeMut>
{
}

impl<U, T> SliceByValueRangeAllMut<T> for U
where
    U: SliceByValueRangeMut<Range<T>>,
    U: SliceByValueRangeMut<RangeFrom<T>>,
    U: SliceByValueRangeMut<RangeFull>,
    U: SliceByValueRangeMut<RangeInclusive<T>>,
    U: SliceByValueRangeMut<RangeTo<T>>,
    U: SliceByValueRangeMut<RangeToInclusive<T>>,
    U: for<'a> SBVRML<'a, Range<T>>,
    U: for<'a> SBVRML<'a, RangeFrom<T>, SliceRangeMut = <U as SBVRML<'a, Range<T>>>::SliceRangeMut>,
    U: for<'a> SBVRML<'a, RangeFull, SliceRangeMut = <U as SBVRML<'a, Range<T>>>::SliceRangeMut>,
    U: for<'a> SBVRML<'a, RangeInclusive<T>, SliceRangeMut = <U as SBVRML<'a, Range<T>>>::SliceRangeMut>,
    U: for<'a> SBVRML<'a, RangeTo<T>, SliceRangeMut = <U as SBVRML<'a, Range<T>>>::SliceRangeMut>,
    U: for<'a> SBVRML<'a, RangeToInclusive<T>, SliceRangeMut = <U as SBVRML<'a, Range<T>>>::SliceRangeMut>,
{
}
