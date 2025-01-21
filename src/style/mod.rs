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
        self.shape.draw_primitive(canvas)(&self.style);
    }
}

#[derive(Default, Debug)]
pub struct DrawStyle<'a> {
    pub fill: Option<Paint<'a>>,
    pub stroke: Option<Paint<'a>>,
    pub stroke_width: Option<f64>,
}

impl<'a> DrawStyle<'a> {
    pub fn from_fill(fill: Fill<'a>) -> DrawStyle<'a> {
        Self {
            fill: Some(fill.into_paint()),
            stroke: None,
            stroke_width: None,
        }
    }

    pub fn from_stroke(stroke: StrokeColor<'a>) -> DrawStyle<'a> {
        Self {
            fill: None,
            stroke: Some(stroke.into_paint()),
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

    pub fn from_style_ref<S: AsDrawStyle>(style: &'a S) -> Self {
        Self {
            fill: style.fill(),
            stroke: style.stroke(),
            stroke_width: style.stroke_width(),
        }
    }
}

pub trait AsDrawStyle {
    fn fill(&self) -> Option<Paint<'_>> {
        None
    }
    fn stroke(&self) -> Option<Paint<'_>> {
        None
    }
    fn stroke_width(&self) -> Option<f64> {
        None
    }
}

impl<T: AsDrawStyle + ?Sized> AsDrawStyle for &'_ T {
    fn fill(&self) -> Option<Paint<'_>> {
        (*self).fill()
    }
    fn stroke(&self) -> Option<Paint<'_>> {
        (*self).stroke()
    }
    fn stroke_width(&self) -> Option<f64> {
        (*self).stroke_width()
    }
}

impl<T: AsDrawStyle + ?Sized> AsDrawStyle for &'_ mut T {
    fn fill(&self) -> Option<Paint<'_>> {
        (**self).fill()
    }
    fn stroke(&self) -> Option<Paint<'_>> {
        (**self).stroke()
    }
    fn stroke_width(&self) -> Option<f64> {
        (**self).stroke_width()
    }
}
