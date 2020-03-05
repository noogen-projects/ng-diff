//! This implementation based on the Hirschberg's algorithm of computing longest
//! common subsequence by linear space, thus algorithm requires O(mn) time and
//! O(m + n) space.
//! http://www.mathcs.emory.edu/~cheung/Courses/323/Syllabus/DynProg/Docs/Hirschberg=Linear-space-LCS.pdf

pub use self::{chars::*, seq::*};

mod chars;
mod seq;

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
    let mut penult = Line::zeroed(b.len() + 1);

    for a in a {
        let mut prev_penult = 0;
        let mut prev_last = 0;
        for (j, b) in b.clone().enumerate() {
            let idx = j + 1;

            let last = if a == b {
                prev_penult + 1
            } else {
                prev_last.max(penult[idx])
            };
            prev_penult = penult[idx];
            prev_last = last;
            penult[idx] = last;
        }
    }
    penult
}

pub fn hirschberg_diff<Line, SeqA, SeqB, SeqC>(a: SeqA, b: SeqB) -> SeqC
where
    SeqA: IntoIterator,
    SeqA::Item: PartialEq + std::fmt::Debug,
    SeqB: IntoIterator<Item = SeqA::Item>,
    SeqA::IntoIter: SequenceIterator,
    SeqB::IntoIter: SequenceIterator,
    SeqC: Difference<SeqA::Item>,
    Line: NwScoreLine,
{
    fn hirschberg_diff_inner<Line, IterA, IterB, SeqC>(mut a: SeqIter<IterA>, b: SeqIter<IterB>, diff: &mut SeqC)
    where
        IterA: SequenceIterator,
        IterA::Item: PartialEq + std::fmt::Debug,
        IterB: SequenceIterator + Iterator<Item = IterA::Item>,
        SeqC: Difference<IterA::Item>,
        Line: NwScoreLine,
    {
        let (a_len, b_len) = (a.len(), b.len());

        if a_len > 0 && b_len == 0 {
            for a_item in a {
                diff.push_first(a_item);
            }
        } else if a_len == 0 && b_len > 0 {
            for b_item in b {
                diff.push_second(b_item);
            }
        } else if a_len > 0 && b_len > 0 {
            if a_len == 1 {
                let a_item = a.next().unwrap();
                let mut found = false;
                for b_item in b {
                    if b_item == a_item && !found {
                        diff.push_both(b_item);
                        found = true;
                    } else {
                        diff.push_second(b_item);
                    }
                }
                if !found {
                    diff.push_first(a_item);
                }
            } else {
                let mid = a_len / 2;
                let score_left: Line = score_last_line(a.clone().take(mid), b.clone());
                let score_right: Line = score_last_line(a.clone().skip(mid).rev(), b.clone().rev());

                let mut k = 0;
                let mut max = 0;
                for j in 0..score_left.len() {
                    let m = score_left[j] + score_right[score_right.len() - 1 - j];
                    if m > max {
                        max = m;
                        k = j;
                    }
                }

                hirschberg_diff_inner::<Line, _, _, _>(a.clone().take(mid), b.clone().take(k), diff);
                hirschberg_diff_inner::<Line, _, _, _>(a.skip(mid), b.skip(k), diff);
            }
        }
    }

    let (a, b) = (a.into_iter(), b.into_iter());
    let (a_len, b_len) = (a.len(), b.len());
    let mut lcs = SeqC::empty();
    hirschberg_diff_inner::<Line, _, _, _>(SeqIter::new(a, a_len), SeqIter::new(b, b_len), &mut lcs);
    lcs
}

pub trait HirschbergAlg {
    type Line: NwScoreLine;

    #[inline]
    fn lcs<SeqA, SeqB, SeqC>(a: SeqA, b: SeqB) -> SeqC
    where
        SeqA: IntoIterator,
        SeqA::Item: PartialEq + std::fmt::Debug,
        SeqB: IntoIterator<Item = SeqA::Item>,
        SeqA::IntoIter: SequenceIterator,
        SeqB::IntoIter: SequenceIterator,
        SeqC: Insert<SeqA::Item>,
    {
        hirschberg_diff::<Self::Line, _, _, Lcs<SeqC>>(a, b).0
    }

    #[inline]
    fn diff<SeqA, SeqB, SeqC>(a: SeqA, b: SeqB) -> SeqC
        where
            SeqA: IntoIterator,
            SeqA::Item: PartialEq + std::fmt::Debug,
            SeqB: IntoIterator<Item = SeqA::Item>,
            SeqA::IntoIter: SequenceIterator,
            SeqB::IntoIter: SequenceIterator,
            SeqC: Insert<DiffItem<SeqA::Item>>,
    {
        hirschberg_diff::<Self::Line, _, _, Diff<SeqC>>(a, b).0
    }
}

pub struct Hirschberg;

impl HirschbergAlg for Hirschberg {
    type Line = Vec<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_last_line() {
        let last: Vec<_> = score_last_line(b"", b"");
        assert_eq!(last, vec![0]);

        let last: Vec<_> = score_last_line(b"AGTACGCA", b"");
        assert_eq!(last, vec![0]);

        let last: Vec<_> = score_last_line(b"", b"TATGC");
        assert_eq!(last, vec![0, 0, 0, 0, 0, 0]);

        let last: Vec<_> = score_last_line(b"AGTACGCA", b"TATGC");
        assert_eq!(last, vec![0, 1, 2, 2, 3, 4]);

        let last: Vec<_> = score_last_line(b"ABCBDAB", b"BDCABA");
        assert_eq!(last, vec![0, 1, 2, 2, 3, 4, 4]);

        let last: Vec<_> = score_last_line(b"BDCABA", b"ABCBDAB");
        assert_eq!(last, vec![0, 1, 2, 2, 3, 3, 4, 4]);

        let last: Vec<_> = score_last_line(&Vec::from("BDCABA"), &b"ABCBDAB"[..]);
        assert_eq!(last, vec![0, 1, 2, 2, 3, 3, 4, 4]);

        let last: Vec<_> = score_last_line(vec!['B', 'D', 'C', 'A', 'B', 'A'], "ABCBDAB".chars_iter());
        assert_eq!(last, vec![0, 1, 2, 2, 3, 3, 4, 4]);
    }

    #[test]
    fn test_hirschberg_lcs() {
        let lcs: Vec<_> = Hirschberg::lcs(b"", b"");
        assert_eq!(lcs, Vec::<&u8>::new());

        let lcs: Vec<_> = Hirschberg::lcs(b"AGTACGCA", b"");
        assert_eq!(lcs, Vec::<&u8>::new());

        let lcs: Vec<_> = Hirschberg::lcs(b"", b"TATGC");
        assert_eq!(lcs, Vec::<&u8>::new());

        let lcs: Vec<_> = Hirschberg::lcs(b"AGTACGCA", b"TATGC");
        assert_eq!(lcs, b"TAGC".iter().collect::<Vec<_>>());

        let lcs: Vec<_> = Hirschberg::lcs(b"ABCBDAB", b"BDCABA");
        assert_eq!(lcs, b"BDAB".iter().collect::<Vec<_>>());

        let lcs: Vec<_> = Hirschberg::lcs(b"BDCABA", b"ABCBDAB");
        assert_eq!(lcs, b"BCBA".iter().collect::<Vec<_>>());

        let lcs: Vec<_> = Hirschberg::lcs(vec!['B', 'D', 'C', 'A', 'B', 'A'], "ABCBDAB".chars_iter());
        assert_eq!(lcs, "BCBA".chars().collect::<Vec<_>>());

        let lcs: Vec<_> = Hirschberg::lcs("abcdfghjqvz".chars_iter(), "abcdefgijkrxyz".chars_iter());
        assert_eq!(lcs, "abcdfgjz".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_hirschberg_diff() {
        let diff: Vec<_> = Hirschberg::diff(b"", b"");
        assert_eq!(diff, Vec::<DiffItem<&u8>>::new());

        let diff: Vec<_> = Hirschberg::diff("AGTACGCA".chars_iter(), "".chars_iter());
        assert_eq!(format_diff(diff), "-A-G-T-A-C-G-C-A".to_string());

        let diff: Vec<_> = Hirschberg::diff("".chars_iter(), "TATGC".chars_iter());
        assert_eq!(format_diff(diff), "+T+A+T+G+C".to_string());

        let diff: Vec<_> = Hirschberg::diff("AGTACGCA".chars_iter(), "TATGC".chars_iter());
        assert_eq!(format_diff(diff), "-A-GTA-C+TGC-A".to_string());

        let diff: Vec<_> = Hirschberg::diff("ABCBDAB".chars_iter(), "BDCABA".chars_iter());
        assert_eq!(format_diff(diff), "-A-B-CBD+CAB+A".to_string());

        let diff: Vec<_> = Hirschberg::diff("BDCABA".chars_iter(), "ABCBDAB".chars_iter());
        assert_eq!(format_diff(diff), "+AB-DC-AB+DA+B".to_string());

        let diff: Vec<_> = Hirschberg::diff("abcdfghjqvz".chars_iter(), "abcdefgijkrxyz".chars_iter());
        assert_eq!(format_diff(diff), "abcd+efg-h+ij-q-v+k+r+x+yz".to_string());
    }

    fn format_diff(diff: Vec<DiffItem<impl std::fmt::Display>>) -> String {
        let mut line = String::new();
        for item in diff {
            match &item {
                DiffItem::First(_) => line.push_str("-"),
                DiffItem::Second(_) => line.push_str("+"),
                _ => (),
            }
            line.push_str(&item.into_inner().to_string());
        }
        line
    }
}
