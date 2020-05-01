use std::hash::Hash;
use syntax::{TextRange, TextUnit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span<T> {
    pub item: T,
    pub start: TextUnit,
    pub end: TextUnit,
}

impl<T> Span<T>
where
    T: std::fmt::Debug + Clone + Hash,
{
    pub fn new(item: T, range: TextRange) -> Self {
        Self {
            item,
            start: range.start(),
            end: range.end(),
        }
    }

    pub fn start(&self) -> TextUnit {
        self.start
    }

    pub fn end(&self) -> TextUnit {
        self.end
    }
}
