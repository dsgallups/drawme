use crate::prelude::*;

mod circle;
pub use circle::*;

mod path;
use nalgebra::Scalar;
pub use path::*;

mod rectangle;
pub use rectangle::*;

pub trait Primitive {
    type Unit: Scalar;
    fn with_style<S>(self, style: S) -> Styled<Self, S>
    where
        Self: Sized,
        S: AsDrawStyle,
    {
        Styled { shape: self, style }
    }

    fn draw_with_style<C, S>(&self, style: S, canvas: &mut C)
    where
        Self: Sized,
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle,
    {
        self.draw_primitive(canvas)(style)
    }

    /// Returns a function that will draw onto the canvas with the provided style.
    fn draw_primitive<'c, C, S>(&'c self, canvas: &'c mut C) -> impl FnMut(S) + 'c
    where
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle,
        Self: Sized;

    /*fn draw_dyn_with_style(&self, style: &dyn AsDrawStyle, canvas: &mut dyn Canvas) {
        self.draw_primitive_boxed(canvas)(style)
    }

    /// Return a function, that when called with a style, will draw to a canvas.
    fn draw_primitive_boxed<'c>(
        &'c self,
        canvas: &'c mut dyn Canvas,
    ) -> Box<dyn FnMut(&dyn AsDrawStyle) + 'c>;*/
}
