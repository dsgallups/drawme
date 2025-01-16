use std::borrow::Cow;

use crate::prelude::*;

mod fill;
pub use fill::*;

mod stroke;
pub use stroke::*;

pub struct Styled<T, S: ?Sized> {
    pub shape: T,
    pub style: S,
}

impl<C, T, S> Draw<C> for Styled<T, S>
where
    T: Primitive,
    C: Canvas,
    S: AsDrawStyle + ?Sized,
{
    fn draw(&self, canvas: &mut C) {
        // style goes first to set values
        // todo: how to deal with style? Maybe we should implement Draw for all primitives
        self.shape.draw_primitive(canvas)(self.style.as_draw_style());
    }
}

#[derive(Default, Debug)]
pub struct DrawStyle<'a> {
    pub fill: Option<Cow<'a, Paint>>,
    pub stroke: Option<Cow<'a, Paint>>,
    pub stroke_width: Option<f64>,
}

impl<'a> DrawStyle<'a> {
    pub fn from_fill(fill: Fill<'a>) -> DrawStyle<'a> {
        Self {
            fill: Some(fill.into_cow()),
            stroke: None,
            stroke_width: None,
        }
    }

    pub fn from_stroke(stroke: StrokeColor<'a>) -> DrawStyle<'a> {
        Self {
            fill: None,
            stroke: Some(stroke.into_cow()),
            stroke_width: None,
        }
    }

    pub fn from_width(width: f64) -> DrawStyle<'a> {
        Self {
            fill: None,
            stroke: None,
            stroke_width: Some(width),
        }
    }
}

pub trait AsDrawStyle {
    fn as_draw_style(&self) -> DrawStyle<'_>;

    fn into_draw_style<'b>(self) -> DrawStyle<'b>
    where
        Self: 'b;
}
