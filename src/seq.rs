use std::{cmp, ops::IndexMut, iter::FusedIterator};

/// The interface of the Needleman-Wunsch score matrix line
pub trait NwScoreLine: IndexMut<usize, Output = usize> {
    fn zeroed(len: usize) -> Self;
    fn len(&self) -> usize;
}

impl NwScoreLine for Vec<usize> {
    fn zeroed(len: usize) -> Self {
        vec![0; len]
    }

    fn len(&self) -> usize {
        self.len()
    }
}

pub trait Insert<T> {
    fn empty() -> Self;
    fn insert(&mut self, item: T);
}

impl<T> Insert<T> for Vec<T> {
    fn empty() -> Self {
        Self::new()
    }

    fn insert(&mut self, item: T) {
        self.push(item)
    }
}

pub trait Difference<T> {
    fn empty() -> Self;
    fn push_first(&mut self, item: T);
    fn push_both(&mut self, item: T);
    fn push_second(&mut self, item: T);
}

#[derive(Clone, Debug)]
pub struct Lcs<T>(pub T);

#[derive(Clone, Debug)]
pub struct Diff<T>(pub T);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffItem<T> {
    First(T),
    Both(T),
    Second(T),
}

impl<T> DiffItem<T> {
    pub fn into_inner(self) -> T {
        match self {
            DiffItem::First(x)
            | DiffItem::Both(x)
            | DiffItem::Second(x) => x,
        }
    }
}

impl<T, I: Insert<T>> Difference<T> for Lcs<I> {
    fn empty() -> Self {
        Self(I::empty())
    }

    fn push_first(&mut self, _item: T) {}

    fn push_both(&mut self, item: T) {
        self.0.insert(item);
    }

    fn push_second(&mut self, _item: T) {}
}

impl<T, I: Insert<DiffItem<T>>> Difference<T> for Diff<I> {
    fn empty() -> Self {
        Self(I::empty())
    }

    fn push_first(&mut self, item: T) {
        self.0.insert(DiffItem::First(item))
    }

    fn push_both(&mut self, item: T) {
        self.0.insert(DiffItem::Both(item))
    }

    fn push_second(&mut self, item: T) {
        self.0.insert(DiffItem::Second(item))
    }
}

pub trait SequenceIterator: ExactSizeIterator + DoubleEndedIterator + Clone {}

impl<T: ExactSizeIterator + DoubleEndedIterator + Clone> SequenceIterator for T {}

#[derive(Clone, Debug)]
pub struct SeqIter<I> {
    iter: I,
    rest: usize,
    reverse: bool,
}

impl<I> SeqIter<I> {
    pub fn new(iter: I, len: usize) -> Self {
        Self {
            iter,
            rest: len,
            reverse: false,
        }
    }
}

impl<I: Iterator> SeqIter<I> {
    #[inline]
    fn own_next(&mut self) -> Option<I::Item> {
        if self.rest > 0 {
            self.rest -= 1;
            self.iter.next()
        } else {
            None
        }
    }

    #[inline]
    fn own_nth(&mut self, n: usize) -> Option<I::Item> {
        if self.rest > n {
            self.rest -= n + 1;
            self.iter.nth(n)
        } else {
            if self.rest > 0 {
                self.iter.nth(self.rest);
                self.rest = 0;
            }
            None
        }
    }
}

impl<I: DoubleEndedIterator + ExactSizeIterator> SeqIter<I> {
    pub fn take(mut self, n: usize) -> Self {
        if n < self.iter.len() {
            if self.reverse {
                self.own_nth(self.iter.len() - n - 1);
            } else {
                self.own_nth_back(self.iter.len() - n - 1);
            }
        }
        self
    }

    pub fn skip(mut self, n: usize) -> Self {
        if n > 0 {
            if self.reverse {
                self.own_nth_back(n - 1);
            } else {
                self.own_nth(n - 1);
            }
        }
        self
    }

    pub fn rev(mut self) -> Self {
        self.reverse = !self.reverse;
        self
    }

    #[inline]
    fn own_next_back(&mut self) -> Option<I::Item> {
        if self.rest == 0 {
            None
        } else {
            let rest = self.rest;
            self.rest -= 1;
            self.iter.nth_back(self.iter.len().saturating_sub(rest))
        }
    }

    #[inline]
    fn own_nth_back(&mut self, n: usize) -> Option<I::Item> {
        let len = self.iter.len();
        if self.rest > n {
            let m = len.saturating_sub(self.rest) + n;
            self.rest -= n + 1;
            self.iter.nth_back(m)
        } else {
            if len > 0 {
                self.iter.nth_back(len - 1);
                self.rest = 0; // todo: ??
            }
            None
        }
    }
}

impl<I> Iterator for SeqIter<I>
where
    I: DoubleEndedIterator + ExactSizeIterator
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.reverse {
            self.own_next_back()
        } else {
            self.own_next()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.rest == 0 {
            return (0, Some(0));
        }

        let (lower, upper) = self.iter.size_hint();
        let lower = cmp::min(lower, self.rest);
        let upper = match upper {
            Some(x) if x < self.rest => Some(x),
            _ => Some(self.rest)
        };

        (lower, upper)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        if self.reverse {
            self.own_nth_back(n)
        } else {
            self.own_nth(n)
        }
    }

    fn fold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, I::Item) -> Acc,
    {
        if self.reverse {
            self.iter.rfold(init, f)
        } else {
            self.iter.fold(init, f)
        }
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<I::Item>
    where
        P: FnMut(&I::Item) -> bool
    {
        if self.reverse {
            self.iter.rfind(predicate)
        } else {
            self.iter.find(predicate)
        }
    }
}

impl<I> DoubleEndedIterator for SeqIter<I>
where
    I: DoubleEndedIterator + ExactSizeIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> {
        if self.reverse {
            self.own_next()
        } else {
            self.own_next_back()
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<I::Item> {
        if self.reverse {
            self.own_nth(n)
        } else {
            self.own_nth_back(n)
        }
    }

    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, I::Item) -> Acc,
    {
        if self.reverse {
            self.iter.fold(init, f)
        } else {
            self.iter.rfold(init, f)
        }
    }

    fn rfind<P>(&mut self, predicate: P) -> Option<I::Item>
    where
        P: FnMut(&I::Item) -> bool
    {
        if self.reverse {
            self.iter.find(predicate)
        } else {
            self.iter.rfind(predicate)
        }
    }
}

impl<I: DoubleEndedIterator + ExactSizeIterator> ExactSizeIterator for SeqIter<I> {}

impl<I: DoubleEndedIterator + ExactSizeIterator + FusedIterator> FusedIterator for SeqIter<I> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_iter() {
        let a = [1, 2, 3];

        let mut iter = SeqIter::new(a.iter(), a.len());
        assert_eq!(3, iter.len());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len());
        assert_eq!(3, iter.len());
        assert_eq!(Some(&3), iter.next_back());
        assert_eq!(Some(&2), iter.next_back());
        assert_eq!(Some(&1), iter.next_back());
        assert_eq!(None, iter.next_back());
        assert_eq!(0, iter.len());


        let mut iter = SeqIter::new(a.iter(), a.len()).take(0);
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).take(1);
        assert_eq!(1, iter.len());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).take(2);
        assert_eq!(2, iter.len());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).take(3);
        assert_eq!(3, iter.len());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).take(4);
        assert_eq!(3, iter.len());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());


        let mut iter = SeqIter::new(a.iter(), a.len()).skip(0);
        assert_eq!(3, iter.len());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).skip(1);
        assert_eq!(2, iter.len());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).skip(2);
        assert_eq!(1, iter.len());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).skip(3);
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).skip(4);
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());


        let mut iter = SeqIter::new(a.iter(), a.len()).rev();
        assert_eq!(3, iter.len());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());


        let mut iter = SeqIter::new(a.iter(), a.len()).skip(1).rev();
        assert_eq!(2, iter.len());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).skip(1).rev().take(1);
        assert_eq!(1, iter.len());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).take(2).rev();
        assert_eq!(2, iter.len());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());

        let mut iter = SeqIter::new(a.iter(), a.len()).take(2).rev().skip(1);
        assert_eq!(1, iter.len());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
    }
}