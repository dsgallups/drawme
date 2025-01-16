use crate::prelude::*;

mod circle;
pub use circle::*;

mod path;
pub use path::*;

mod rectangle;
pub use rectangle::*;

pub trait Primitive {
    fn with_style<S>(self, style: S) -> Styled<Self, S>
    where
        Self: Sized,
        S: AsDrawStyle,
    {
        Styled { shape: self, style }
    }

    fn draw_with_style<C, S>(&self, style: &S, canvas: &mut C)
    where
        Self: Sized,
        C: Canvas,
        S: AsDrawStyle + ?Sized,
    {
        self.draw_primitive(canvas)(style.as_draw_style())
    }

    /// non-dyn function for primitive. Highly recommended to implement this function.
    fn draw_primitive<'c, C>(&'c self, canvas: &'c mut C) -> impl FnMut(DrawStyle<'_>) + 'c
    where
        C: Canvas,
        Self: Sized,
    {
        self.draw_primitive_boxed(canvas)
    }

    fn draw_dyn_with_style(&self, style: &dyn AsDrawStyle, canvas: &mut dyn Canvas) {
        self.draw_primitive_boxed(canvas)(style.as_draw_style())
    }

    /// Return a function, that when called with a style, will draw to a canvas.
    fn draw_primitive_boxed<'c>(
        &'c self,
        canvas: &'c mut dyn Canvas,
    ) -> Box<dyn FnMut(DrawStyle<'_>) + 'c>;
}

pub trait DynPrimitive {
    fn draw_dyn_primitive(&self, canvas: &mut dyn Canvas) -> Box<dyn FnMut(DrawStyle<'_>)>;
}
