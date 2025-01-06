use std::borrow::Cow;
mod builder;
pub use builder::*;

pub mod font;

pub struct Text<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn builder<'b>() -> TextBuilder<'b> {
        TextBuilder::default()
    }
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self { inner: text.into() }
    }
}
