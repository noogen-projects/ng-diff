//! This implementation based on the Hirschberg's algorithm of computing longest
//! common subsequence by linear space, thus algorithm requires O(mn) time and
//! O(m + n) space.
//! http://www.mathcs.emory.edu/~cheung/Courses/323/Syllabus/DynProg/Docs/Hirschberg=Linear-space-LCS.pdf

use std::ops::IndexMut;

/// The interface of the Needleman-Wunsch score matrix line
pub trait NwScoreLine: IndexMut<usize, Output = usize> {
    fn new(len: usize) -> Self;
}

impl NwScoreLine for Vec<usize> {
    fn new(len: usize) -> Self {
        vec![0; len]
    }
}

/// This function returns the last line of the Needleman-Wunsch score matrix
pub fn score_last_line<L, T>(a: &[T], b: &[T]) -> L
where
    T: PartialEq,
    L: NwScoreLine,
{
    let mut penult = L::new(b.len());
    let mut last = L::new(b.len());

    for i in 0..a.len() {
        let tmp = penult;
        penult = last;
        last = tmp;
        for j in 0..b.len() {
            if a[i] == b[j] {
                last[j] = if j > 0 { penult[j - 1] } else { 0 } + 1;
            } else {
                last[j] = if j > 0 { last[j - 1] } else { 0 }.max(penult[j]);
            }
        }
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_last_line() {
        let last: Vec<_> = score_last_line(b"", b"");
        assert_eq!(last, vec![]);

        let last: Vec<_> = score_last_line(b"AGTACGCA", b"");
        assert_eq!(last, vec![]);

        let last: Vec<_> = score_last_line(b"", b"TATGC");
        assert_eq!(last, vec![0, 0, 0, 0, 0]);

        let last: Vec<_> = score_last_line(b"AGTACGCA", b"TATGC");
        assert_eq!(last, vec![1, 2, 2, 3, 4]);

        let last: Vec<_> = score_last_line(b"ABCBDAB", b"BDCABA");
        assert_eq!(last, vec![1, 2, 2, 3, 4, 4]);

        let last: Vec<_> = score_last_line(b"BDCABA", b"ABCBDAB");
        assert_eq!(last, vec![1, 2, 2, 3, 3, 4, 4]);
    }
}
