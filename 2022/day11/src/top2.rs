

pub trait Top2
where
    Self: IntoIterator,
{
    fn top2(self) -> (Option<Self::Item>, Option<Self::Item>);
}

impl<I, I1, I2> Top2 for I
where
    I: IntoIterator<Item=I1, IntoIter=I2>,
    I2: Iterator<Item=I1>,
    I1: Copy + Ord,
{
    fn top2(self) -> (Option<Self::Item>, Option<Self::Item>) {
        let acc = (None, None);

        self.into_iter().fold(acc, |acc, item| {
            if acc.0.is_none() || (acc.0.unwrap() < item) {
                return (Some(item), acc.0, );
            } else if acc.1.is_none() || (acc.1.unwrap() < item) {
                return (acc.0, Some(item));
            }
            acc
        })
    }
}