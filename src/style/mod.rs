use crate::prelude::*;

mod fill;
pub use fill::*;

mod stroke;
use nalgebra::Scalar;
pub use stroke::*;

pub struct Styled<T, S: ?Sized> {
    pub shape: T,
    pub style: S,
}

impl<U, C, T, S> Draw<C> for Styled<T, S>
where
    U: Scalar,
    T: Primitive<Unit = U>,
    C: Canvas<Unit = T::Unit>,
    S: AsDrawStyle<T::Unit>,
{
    fn draw(&self, canvas: &mut C) {
        // style goes first to set values
        // todo: how to deal with style? Maybe we should implement Draw for all primitives
        self.shape.draw_primitive(canvas)(&self.style);
    }
}

#[derive(Default, Debug)]
pub struct DrawStyle<'a, Unit: Scalar = f64> {
    pub fill: Option<Paint<'a, Unit>>,
    pub stroke: Option<Paint<'a, Unit>>,
    pub stroke_width: Option<Unit>,
}

impl<'a, Unit: Scalar> DrawStyle<'a, Unit> {
    pub fn from_fill(fill: Fill<'a, Unit>) -> DrawStyle<'a, Unit> {
        Self {
            fill: Some(fill.into_paint()),
            stroke: None,
            stroke_width: None,
        }
    }

    pub fn from_stroke(stroke: StrokeColor<'a, Unit>) -> DrawStyle<'a, Unit> {
        Self {
            fill: None,
            stroke: Some(stroke.into_paint()),
            stroke_width: None,
        }
    }

    pub fn from_style_ref<S: AsDrawStyle<Unit>>(style: &'a S) -> Self {
        Self {
            fill: style.fill(),
            stroke: style.stroke(),
            stroke_width: style.stroke_width(),
        }
    }

    pub fn from_width(width: Unit) -> DrawStyle<'a, Unit> {
        Self {
            fill: None,
            stroke: None,
            stroke_width: Some(width),
        }
    }
}

impl<Unit: Scalar> DrawStyle<'static, Unit> {
    pub fn from_style<S: AsDrawStyle<Unit>>(style: S) -> Self {
        Self {
            fill: style.fill().map(|s| s.into_owned()),
            stroke: style.stroke().map(|s| s.into_owned()),
            stroke_width: style.stroke_width().clone(),
        }
    }
}
impl<'a, Unit: Scalar> DrawStyle<'a, Unit> {
    pub const fn clone_shallow(&self) -> DrawStyle<'_, Unit> {
        DrawStyle {
            stroke_width: self.stroke_width,
            stroke: match self.stroke {
                None => None,
                Some(s) => Some(s.clone_shallow()),
            },
            fill: match self.fill {
                None => None,
                Some(s) => Some(s.clone_shallow()),
            },
        }
    }
}

pub trait AsDrawStyle<Unit: Scalar = f64> {
    fn fill(&self) -> Option<Paint<'_, Unit>> {
        None
    }
    fn stroke(&self) -> Option<Paint<'_, Unit>> {
        None
    }
    fn stroke_width(&self) -> Option<Unit> {
        None
    }
}

impl<Unit: Scalar, T: AsDrawStyle<Unit> + ?Sized> AsDrawStyle<Unit> for &T {
    fn fill(&self) -> Option<Paint<'_, Unit>> {
        (*self).fill()
    }
    fn stroke(&self) -> Option<Paint<'_, Unit>> {
        (*self).stroke()
    }
    fn stroke_width(&self) -> Option<Unit> {
        (*self).stroke_width()
    }
}

impl<Unit: Scalar, T: AsDrawStyle<Unit> + ?Sized> AsDrawStyle<Unit> for &'_ mut T {
    fn fill(&self) -> Option<Paint<'_, Unit>> {
        (**self).fill()
    }
    fn stroke(&self) -> Option<Paint<'_, Unit>> {
        (**self).stroke()
    }
    fn stroke_width(&self) -> Option<Unit> {
        (**self).stroke_width()
    }
}
