use crate::RegexBuilder;

#[must_use]
#[derive(Clone, Copy)]
pub struct And<A, B>(pub(crate) A, pub(crate) B);

impl<A, B> RegexBuilder for And<A,B> where A: RegexBuilder, B: RegexBuilder {
    fn build(self) -> String {
        let lhs = self.0.build();
        let rhs = self.1.build();
        
        format!("{lhs}{rhs}")
    }
}

#[must_use]
#[derive(Clone, Copy)]
pub struct Or<A, B>(pub(crate) A, pub(crate) B);

impl<A, B> RegexBuilder for Or<A, B> where A: RegexBuilder, B: RegexBuilder {
    fn build(self) -> String {
        let lhs = self.0.build();
        let rhs = self.1.build();
        format!("{lhs}|{rhs}")
    }
}

#[derive(Clone, Copy)]
pub struct Repeated<A: RegexBuilder> {
    inner: A,
    min: usize,
    max: Option<usize>,
}

impl<A: RegexBuilder> Repeated<A> {
    pub(crate) fn with_defaults(inner: A) -> Self {
        Self {
            inner,
            min: 0,
            max: None,
        }
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }

    pub fn at_most(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    pub fn exactly(mut self, n: usize) -> Self {
        self.min = n;
        self.max = Some(n);
        self
    }
}

impl<A: RegexBuilder> RegexBuilder for Repeated<A> {
    fn build(self) -> String {
        let inner = self.inner.build();
        let min = self.min;
        
        match self.max {
            Some(max) if min == max => format!("{inner}{{{min}}}"),
            Some(max) => format!("{inner}{{{min},{max}}}"),
            None => format!("{inner}{{{min},}}"),
        }
    }
}
