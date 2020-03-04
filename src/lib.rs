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
pub fn score_last_line<SeqA, SeqB, Line>(a: SeqA, b: SeqB) -> Line
where
    SeqA: IntoIterator,
    SeqB: IntoIterator,
    SeqA::IntoIter: ExactSizeIterator,
    SeqB::IntoIter: ExactSizeIterator + Clone,
    SeqA::Item: PartialEq<SeqB::Item>,
    Line: NwScoreLine,
{
    let (a, b) = (a.into_iter(), b.into_iter());
    let mut penult = Line::new(b.len());
    let mut last = Line::new(b.len());

    for a in a {
        let tmp = penult;
        penult = last;
        last = tmp;
        let mut prev_penult = 0;
        let mut prev_last = 0;
        for (j, b) in b.clone().enumerate() {
            if a == b {
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

        let last: Vec<_> = score_last_line(&Vec::from("BDCABA"), &b"ABCBDAB"[..]);
        assert_eq!(last, vec![1, 2, 2, 3, 3, 4, 4]);
    }
}
