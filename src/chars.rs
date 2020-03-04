use std::iter::FusedIterator;

/// An `ExactSizeIterator` implementation over the `char`s of
/// a string slice.
#[derive(Debug, Clone)]
pub struct Chars<'a> {
    chars: std::str::Chars<'a>,
}

impl Iterator for Chars<'_> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chars.size_hint()
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

impl DoubleEndedIterator for Chars<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<char> {
        self.chars.next_back()
    }
}

impl FusedIterator for Chars<'_> {}

impl ExactSizeIterator for Chars<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.chars.clone().count()
    }

    // Use this impl when `is_empty` will be stabilized:
    // https://github.com/rust-lang/rust/issues/35428
    //
    // #[inline]
    // fn is_empty(&self) -> bool {
    //     self.chars.size_hint().1
    //         .map(|len| len == 0)
    //         .unwrap_or_default()
    // }
}

impl<'a> Chars<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars()
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }
}

impl<'a> From<&'a str> for Chars<'a> {
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}

impl<'a> From<&'a mut str> for Chars<'a> {
    fn from(source: &'a mut str) -> Self {
        Self::new(source)
    }
}

impl<'a> From<&'a String> for Chars<'a> {
    fn from(source: &'a String) -> Self {
        Self::new(source.as_str())
    }
}

impl<'a> From<&'a mut String> for Chars<'a> {
    fn from(source: &'a mut String) -> Self {
        Self::new(source.as_str())
    }
}