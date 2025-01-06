use crate::prelude::*;
use std::borrow::Cow;

#[derive(Default)]
pub struct TextBuilder<'a> {
    inner: Cow<'a, str>,
}

impl<'a> TextBuilder<'a> {
    pub fn with_str(mut self, text: impl Into<Cow<'a, str>>) -> Self {
        self.inner = text.into();
        self
    }

    pub fn build(self) -> Text<'a> {
        Text { inner: self.inner }
    }
}
