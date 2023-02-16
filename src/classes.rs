use crate::RegexBuilder;
use std::ops::RangeInclusive;

#[must_use]
#[derive(Clone, Copy)]
pub struct CharacterClass<A> {
    pub(crate) inner: A,
    pub(crate) negated: bool,
}

impl<A: RegexBuilder> CharacterClass<A> {
    pub fn not(mut self) -> Self {
        self.negated = true;
        self
    }
}

impl<A: RegexBuilder> RegexBuilder for CharacterClass<A> {
    fn build(self) -> String {
        let inner = self.inner.build();
        if self.negated {
            format!("[^{inner}]")
        } else {
            format!("[{inner}]")
        }
    }
}

#[must_use]
#[derive(Clone)]
pub struct InCharRange(RangeInclusive<char>);

impl RegexBuilder for InCharRange {
    fn build(self) -> String {
        let start = *self.0.start();
        let end = *self.0.end();

        format!("{start}-{end}")
    }
}

pub fn in_between(range: RangeInclusive<char>) -> InCharRange {
    InCharRange(range)
}
