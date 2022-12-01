pub trait Top3
where
    Self: Iterator,
{
    fn top3(self) -> (Option<Self::Item>, Option<Self::Item>, Option<Self::Item>);
}

impl<I> Top3 for I
where
    I: Iterator,
    Self::Item: Copy + Ord,
{
    fn top3(self) -> (Option<Self::Item>, Option<Self::Item>, Option<Self::Item>) {
        let acc = (None, None, None);

        self.fold(acc, |acc, item| {
            if acc.0.is_none() || (acc.0.unwrap() < item) {
                return (Some(item), acc.0, acc.1);
            } else if acc.1.is_none() || (acc.1.unwrap() < item) {
                return (acc.0, Some(item), acc.1);
            } else if acc.2.is_none() || (acc.2.unwrap() < item) {
                return (acc.0, acc.1, Some(item));
            }
            acc
        })
    }
}
