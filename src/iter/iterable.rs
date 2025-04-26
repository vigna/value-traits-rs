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
    // this can always be implemented as self.iter().skip(idx)
    // but the implementer might know better ways to do it
    fn iter_from(&self, idx: usize) -> Self::IterFrom<'_>;
}
