use crate::{RegexBuilder, primitives::raw};

pub fn any() -> impl RegexBuilder {
    raw(".")
}

pub fn bell() -> impl RegexBuilder {
    raw(r"\b")
}

pub fn horizontal_tab() -> impl RegexBuilder {
    raw(r"\t")
}

pub fn vertical_tab() -> impl RegexBuilder {
    raw(r"\v")
}

pub fn newline() -> impl RegexBuilder {
    raw(r"\n")
}

pub fn whitespace() -> impl RegexBuilder {
    raw(r"\s")
}

pub fn digit() -> impl RegexBuilder {
    raw(r"\d")
}

pub fn non_digit() -> impl RegexBuilder {
    raw(r"\D")
}

pub fn non_whitespace() -> impl RegexBuilder {
    raw(r"\S")
}

pub fn word_character() -> impl RegexBuilder {
    raw(r"\w")
}

pub fn non_word_character() -> impl RegexBuilder {
    raw(r"\W")
}
