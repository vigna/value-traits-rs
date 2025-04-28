pub trait Iterable {
    type Item;
    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;
    fn iter(&self) -> Self::Iter<'_>;
}

pub trait IterableFrom: Iterable {
    type IterFrom<'a>: Iterator<Item = <Self as Iterable>::Item>
    where
        Self: 'a;
    fn iter_from(&self, idx: usize) -> Self::IterFrom<'_>;
}
