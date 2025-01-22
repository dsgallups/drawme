use crate::prelude::*;

mod circle;
pub use circle::*;

mod path;
use nalgebra::Scalar;
pub use path::*;

mod rectangle;
pub use rectangle::*;

pub trait Primitive: Sized {
    type Unit: Scalar;
    fn with_style<S>(self, style: S) -> Styled<Self, S>
    where
        S: AsDrawStyle<Self::Unit>,
    {
        Styled { shape: self, style }
    }

    fn draw_with_style<C, S>(&self, style: S, canvas: &mut C)
    where
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle<Self::Unit>,
    {
        self.draw_primitive(canvas)(style)
    }

    /// Returns a function that will draw onto the canvas with the provided style.
    fn draw_primitive<'c, C, S>(&'c self, canvas: &'c mut C) -> impl FnMut(&S) + 'c
    where
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle<Self::Unit> + ?Sized;
}
