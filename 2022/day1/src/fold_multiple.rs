use itertools::FoldWhile;
use itertools::Itertools;
use std::iter::Peekable;

pub struct FoldMultipleIter<I, F, B>
where
    I: Iterator,
{
    inner: Peekable<I>,
    f: F,
    init_acc: B,
}

impl<I, F, B> Iterator for FoldMultipleIter<I, F, B>
where
    I: Iterator,
    F: FnMut(B, I::Item) -> FoldWhile<B>,
    B: Clone + Copy,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.peek().is_some() {
            Some(
                self.inner
                    .fold_while(self.init_acc, &mut self.f)
                    .into_inner(),
            )
        } else {
            None
        }
    }
}

pub trait FoldMultiple<I, F, B>
where
    I: Iterator,
{
    fn fold_multiple(self, init: B, f: F) -> FoldMultipleIter<I, F, B>;
}

impl<I, F, B> FoldMultiple<I, F, B> for I
where
    I: Iterator,
{
    fn fold_multiple(self, init: B, f: F) -> FoldMultipleIter<I, F, B> {
        FoldMultipleIter {
            inner: self.peekable(),
            f,
            init_acc: init,
        }
    }
}
