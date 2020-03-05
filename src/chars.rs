use std::iter::FusedIterator;

pub trait AsCharsIter {
    fn chars_iter(&self) -> CharsIter;
}

impl AsCharsIter for str {
    fn chars_iter(&self) -> CharsIter {
        self.into()
    }
}

impl AsCharsIter for String {
    fn chars_iter(&self) -> CharsIter {
        self.into()
    }
}

/// An `ExactSizeIterator` implementation over the `char`s of
/// a string slice.
#[derive(Debug, Clone)]
pub struct CharsIter<'a> {
    chars: std::str::Chars<'a>,
    len: usize,
}

impl Iterator for CharsIter<'_> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        if self.len > 0 {
            self.len -= 1;
        }
        self.chars.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn count(self) -> usize {
        self.chars.count()
    }

    #[inline]
    fn last(self) -> Option<char> {
        self.chars.last()
    }
}

impl DoubleEndedIterator for CharsIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<char> {
        if self.len > 0 {
            self.len -= 1;
        }
        self.chars.next_back()
    }
}

impl FusedIterator for CharsIter<'_> {}

impl ExactSizeIterator for CharsIter<'_> {}

impl<'a> CharsIter<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        let chars = source.chars();
        let len = chars.clone().count();
        Self {
            chars,
            len,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }
}

impl<'a> From<&'a str> for CharsIter<'a> {
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}

impl<'a> From<&'a mut str> for CharsIter<'a> {
    fn from(source: &'a mut str) -> Self {
        Self::new(source)
    }
}

impl<'a> From<&'a String> for CharsIter<'a> {
    fn from(source: &'a String) -> Self {
        Self::new(source.as_str())
    }
}

impl<'a> From<&'a mut String> for CharsIter<'a> {
    fn from(source: &'a mut String) -> Self {
        Self::new(source.as_str())
    }
}