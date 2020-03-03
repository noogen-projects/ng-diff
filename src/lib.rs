//! This implementation based on the Hirschberg's algorithm of computing longest
//! common subsequence by linear space, thus algorithm requires O(mn) time and
//! O(m + n) space.
//! http://www.mathcs.emory.edu/~cheung/Courses/323/Syllabus/DynProg/Docs/Hirschberg=Linear-space-LCS.pdf

use std::ops::{Index, IndexMut};

/// The interface of the Needleman-Wunsch score matrix line
pub trait NwScoreLine: IndexMut<usize, Output = usize> {
    fn new(len: usize) -> Self;
}

impl NwScoreLine for Vec<usize> {
    fn new(len: usize) -> Self {
        vec![0; len]
    }
}

pub trait SequenceCollection: Index<usize> {
    fn len(&self) -> usize;
}

impl<T> SequenceCollection for [T] {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> SequenceCollection for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

/// This function returns the last line of the Needleman-Wunsch score matrix
pub fn score_last_line<SeqA, SeqB, Line>(a: &SeqA, b: &SeqB) -> Line
where
    SeqA: SequenceCollection + ?Sized,
    SeqB: SequenceCollection + ?Sized,
    SeqA::Output: PartialEq<SeqB::Output>,
    Line: NwScoreLine,
{
    let mut penult = Line::new(b.len());
    let mut last = Line::new(b.len());

    for i in 0..a.len() {
        let tmp = penult;
        penult = last;
        last = tmp;
        let mut prev_penult = 0;
        let mut prev_last = 0;
        for j in 0..b.len() {
            if a[i] == b[j] {
                last[j] = prev_penult + 1;
            } else {
                last[j] = prev_last.max(penult[j]);
            }
            prev_penult = penult[j];
            prev_last = last[j];
        }
    }
    last
}

pub fn score_last_line_for_slices<T, Line>(a: &[T], b: &[T]) -> Line
where
    T: PartialEq,
    Line: NwScoreLine,
{
    score_last_line(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_last_line() {
        let last: Vec<_> = score_last_line_for_slices(b"", b"");
        assert_eq!(last, vec![]);

        let last: Vec<_> = score_last_line_for_slices(b"AGTACGCA", b"");
        assert_eq!(last, vec![]);

        let last: Vec<_> = score_last_line_for_slices(b"", b"TATGC");
        assert_eq!(last, vec![0, 0, 0, 0, 0]);

        let last: Vec<_> = score_last_line_for_slices(b"AGTACGCA", b"TATGC");
        assert_eq!(last, vec![1, 2, 2, 3, 4]);

        let last: Vec<_> = score_last_line_for_slices(b"ABCBDAB", b"BDCABA");
        assert_eq!(last, vec![1, 2, 2, 3, 4, 4]);

        let last: Vec<_> = score_last_line_for_slices(b"BDCABA", b"ABCBDAB");
        assert_eq!(last, vec![1, 2, 2, 3, 3, 4, 4]);

        let last: Vec<_> = score_last_line(&Vec::from("BDCABA"), &b"ABCBDAB"[..]);
        assert_eq!(last, vec![1, 2, 2, 3, 3, 4, 4]);
    }
}
