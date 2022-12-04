pub trait Contains {
    fn contains(&self, other: &Self) -> bool;

    fn any_contains_other(lhs: &Self, rhs: &Self) -> bool {
        lhs.contains(rhs) || rhs.contains(lhs)
    }
}

impl<T> Contains for (T, T)
where
    T: PartialOrd,
{
    fn contains(&self, other: &Self) -> bool {
        (self.0 <= other.0) && (self.1 >= other.1)
    }
}
