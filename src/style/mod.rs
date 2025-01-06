use crate::prelude::*;

mod fill;
pub use fill::*;

mod stroke;
pub use stroke::*;

pub struct Styled<T, S> {
    pub shape: T,
    pub style: S,
}

impl<C, T, S> Draw<C> for Styled<T, S>
where
    C: Canvas + ?Sized,
    T: Draw<C>,
    S: Draw<C>,
{
    fn draw(&self, canvas: &mut C) {
        // style goes first to set values
        self.style.draw(canvas);
        self.shape.draw(canvas);
    }
}
