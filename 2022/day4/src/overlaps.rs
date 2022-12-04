pub trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T> Overlaps for (T, T)
where
    T: PartialOrd,
{
    fn overlaps(&self, other: &Self) -> bool {
        ((self.0 < other.0) && (other.0 <= self.1)
            || ((other.0 < self.0) && (self.0 <= other.1))
            || self.0 == other.0)
    }
}
