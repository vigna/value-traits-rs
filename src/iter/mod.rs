pub trait IterableByValue {
    type Item;
    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;
    fn iter_value(&self) -> Self::Iter<'_>;
}

pub trait IterableByValueFrom: IterableByValue {
    type IterFrom<'a>: Iterator<Item = <Self as IterableByValue>::Item>
    where
        Self: 'a;
    fn iter_value_from(&self, idx: usize) -> Self::IterFrom<'_>;
}
