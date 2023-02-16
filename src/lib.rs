use combinators::{Or, Repeated};
use primitives::{Optional, Grouped, NonCapturingGroup, StartAnchored};
use crate::classes::CharacterClass;
use crate::combinators::And;

mod classes;
mod combinators;
mod primitives;
mod specials;

pub mod prelude {
    pub use crate::RegexBuilder;
    pub use crate::classes::in_between;
    pub use crate::primitives::{is_char, raw};
    pub use crate::specials::*;
}

pub trait RegexBuilder {
    fn and<R>(self, other: R) -> And<Self, R>
    where
        R: RegexBuilder,
        Self: Sized,
    {
        And(self, other)
    }

    fn or<R>(self, other: R) -> Or<Self, R>
    where
        R: RegexBuilder,
        Self: Sized,
    {
        Or(self, other)
    }

    fn or_not(self) -> Optional<Self> where Self: Sized {
        Optional(self)
    }

    fn repeated(self) -> Repeated<Self> where Self: Sized {
        Repeated::with_defaults(self)
    }

    fn as_character_class(self) -> CharacterClass<Self>
    where
        Self: Sized,
    {
        CharacterClass {
            inner: self,
            negated: false,
        }
    }

    fn grouped(self) -> Grouped<Self> where Self: Sized {
        Grouped::new(self)
    }

    fn grouped_as(self, name: &str) -> Grouped<Self> where Self: Sized {
        Grouped::with_name(self, name)
    }

    fn grouped_non_capturing(self) -> NonCapturingGroup<Self> where Self: Sized {
        NonCapturingGroup(self)
    }

    fn anchor_start(self) -> StartAnchored<Self> where Self: Sized {
        StartAnchored(self)
    }

    fn anchor_end(self) -> StartAnchored<Self> where Self: Sized {
        StartAnchored(self)
    }

    fn build(self) -> String;
}
