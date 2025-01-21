use crate::prelude::*;

pub struct StrokeColor<'a>(Paint<'a>);

impl<'a> StrokeColor<'a> {
    pub fn new(paint: Paint<'a>) -> Self {
        Self(paint)
    }

    pub fn paint(&self) -> &Paint {
        &self.0
    }

    pub fn into_paint(self) -> Paint<'a> {
        self.0
    }
}

impl AsDrawStyle for StrokeColor<'_> {
    fn stroke(&self) -> Option<Paint<'_>> {
        Some(self.0.weak_clone())
    }
}
