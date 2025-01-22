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

impl<C, T, S> Draw<C> for Styled<T, S>
where
    T: Primitive,
    C: Canvas<Unit = T::Unit>,
    S: AsDrawStyle<Unit = T::Unit>,
{
    fn draw(&self, canvas: &mut C) {
        // style goes first to set values
        // todo: how to deal with style? Maybe we should implement Draw for all primitives
        self.shape.draw_primitive(canvas)(&self.style);
    }
}

#[derive(Debug, Clone)]
pub struct DrawStyle<'a, Unit: Scalar = f64> {
    pub fill: Option<Paint<'a, Unit>>,
    pub stroke: Option<Paint<'a, Unit>>,
    pub stroke_width: Option<Unit>,
}

impl<U: Scalar> Default for DrawStyle<'_, U> {
    fn default() -> Self {
        Self {
            fill: None,
            stroke: None,
            stroke_width: None,
        }
    }
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

    pub fn fill(&self) -> Option<Paint<'_, Unit>> {
        self.fill.as_ref().map(|p| p.clone_shallow())
    }
    pub fn stroke(&self) -> Option<Paint<'_, Unit>> {
        self.stroke.as_ref().map(|p| p.clone_shallow())
    }

    pub fn from_style_ref<S: AsDrawStyle<Unit = Unit>>(style: &'a S) -> Self {
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

    pub fn set_fill(&mut self, fill: Option<Paint<'a, Unit>>) -> &mut Self {
        self.fill = fill;
        self
    }

    pub fn set_stroke(&mut self, stroke: Option<Paint<'a, Unit>>) -> &mut Self {
        self.stroke = stroke;
        self
    }

    pub fn set_stroke_width(&mut self, stroke_width: Option<Unit>) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn combine_styles<'res, 'par>(
        &self,
        parent_style: &DrawStyle<'par, Unit>,
    ) -> DrawStyle<'res, Unit>
    where
        'a: 'res,
        'par: 'res,
    {
        todo!()
    }
}

impl<Unit: Scalar> DrawStyle<'static, Unit> {
    pub fn from_style<S: AsDrawStyle<Unit = Unit>>(style: S) -> Self {
        Self {
            fill: style.fill().map(|s| s.into_owned()),
            stroke: style.stroke().map(|s| s.into_owned()),
            stroke_width: style.stroke_width().clone(),
        }
    }
}
impl<'a, Unit: Scalar + Copy> DrawStyle<'a, Unit> {
    pub fn stroke_width(&self) -> Option<Unit> {
        self.stroke_width
    }
    pub const fn clone_shallow(&self) -> DrawStyle<'_, Unit> {
        DrawStyle {
            stroke_width: self.stroke_width,
            stroke: match &self.stroke {
                None => None,
                Some(s) => Some(s.clone_shallow()),
            },
            fill: match &self.fill {
                None => None,
                Some(s) => Some(s.clone_shallow()),
            },
        }
    }
}

pub trait AsDrawStyle {
    type Unit: Scalar;
    fn fill(&self) -> Option<Paint<'_, Self::Unit>> {
        None
    }
    fn stroke(&self) -> Option<Paint<'_, Self::Unit>> {
        None
    }
    fn stroke_width(&self) -> Option<Self::Unit> {
        None
    }
}

impl<T: AsDrawStyle + ?Sized> AsDrawStyle for &T {
    type Unit = T::Unit;
    fn fill(&self) -> Option<Paint<'_, Self::Unit>> {
        (*self).fill()
    }
    fn stroke(&self) -> Option<Paint<'_, Self::Unit>> {
        (*self).stroke()
    }
    fn stroke_width(&self) -> Option<Self::Unit> {
        (*self).stroke_width()
    }
}

impl<T: AsDrawStyle + ?Sized> AsDrawStyle for &'_ mut T {
    type Unit = T::Unit;

    fn fill(&self) -> Option<Paint<'_, Self::Unit>> {
        (**self).fill()
    }
    fn stroke(&self) -> Option<Paint<'_, Self::Unit>> {
        (**self).stroke()
    }
    fn stroke_width(&self) -> Option<Self::Unit> {
        (**self).stroke_width()
    }
}
