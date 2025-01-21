use crate::prelude::*;

pub struct Fill<'a>(Paint<'a>);

impl<'a> Fill<'a> {
    pub fn new(paint: impl Into<Paint<'a>>) -> Self {
        Self(paint.into())
    }

    pub fn paint<'slf>(&'slf self) -> &'slf Paint<'a> {
        &self.0
    }

    pub fn into_paint(self) -> Paint<'a> {
        self.0
    }
}

impl AsDrawStyle for Fill<'_> {
    fn fill(&self) -> Option<Paint<'_>> {
        Some(self.0.clone_shallow())
    }
}
