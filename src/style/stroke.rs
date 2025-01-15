use std::borrow::Cow;

use crate::prelude::*;

pub struct StrokeColor<'a>(Cow<'a, Paint>);

impl<'a> StrokeColor<'a> {
    pub fn new(paint: impl Into<Cow<'a, Paint>>) -> Self {
        Self(paint.into())
    }

    pub fn paint_mut(&mut self) -> &mut Paint {
        self.0.to_mut()
    }

    pub fn paint(&self) -> &Paint {
        &self.0
    }

    pub fn into_cow(self) -> Cow<'a, Paint> {
        self.0
    }
    pub fn into_paint(self) -> Paint {
        self.0.into_owned()
    }
}

impl<C: Canvas + ?Sized> Draw<C> for StrokeColor<'_> {
    fn draw(&self, canvas: &mut C) {
        canvas.set_stroke_color(Some(self.paint()));
    }
}
