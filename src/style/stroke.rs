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

impl AsDrawStyle for StrokeColor<'_> {
    fn as_draw_style(&self) -> DrawStyle<'_> {
        DrawStyle {
            stroke: Some(Cow::Borrowed(self.paint())),
            ..Default::default()
        }
    }
    fn into_draw_style<'b>(self) -> DrawStyle<'b>
    where
        Self: 'b,
    {
        DrawStyle::from_stroke(self)
    }
}
