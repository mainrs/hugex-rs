use crate::RegexBuilder;

pub fn is_char(c: char) -> impl RegexBuilder {
    raw(&c.to_string())
}

pub struct Raw(String);

impl RegexBuilder for Raw {
    fn build(self) -> String {
        self.0
    }
}

pub fn raw(value: &str) -> Raw {
    Raw(value.to_owned())
}

pub struct Optional<A>(pub(crate) A);

impl<A: RegexBuilder> RegexBuilder for Optional<A> {
    fn build(self) -> String {
        let inner = self.0.build();
        format!("{inner}?")
    }
}

#[must_use]
pub struct Grouped<A> {
    inner: A,
    name: Option<String>,
}

impl<A: RegexBuilder> Grouped<A> {
    pub(crate) fn new(inner: A) -> Self {
        Self { inner, name: None }
    }

    pub(crate) fn with_name(inner: A, name: &str) -> Self {
        Self {
            inner,
            name: Some(name.to_owned()),
        }
    }
}

impl<A: RegexBuilder> RegexBuilder for Grouped<A> {
    fn build(self) -> String {
        let inner = self.inner.build();
        match self.name {
            Some(name) => format!("(?P<{name}>{inner})"),
            None => format!("({inner})"),
        }
    }
}

pub struct NonCapturingGroup<A>(pub(crate) A);

impl<A: RegexBuilder> RegexBuilder for NonCapturingGroup<A> {
    fn build(self) -> String {
        let inner = self.0.build();
        format!("(?:{inner})")
    }
}

pub struct StartAnchored<A>(pub(crate) A);

impl<A: RegexBuilder> RegexBuilder for StartAnchored<A> {
    fn build(self) -> String {
        let inner = self.0.build();
        format!("^{inner}")
    }
}

pub struct EndAnchored<A>(pub(crate) A);

impl<A: RegexBuilder> RegexBuilder for EndAnchored<A> {
    fn build(self) -> String {
        let inner = self.0.build();
        format!("{inner}$")
    }
}
