use crate::prelude::*;

mod circle;
pub use circle::*;

mod path;
pub use path::*;

pub trait Primitive<S>: Sized {
    fn with_style(self, style: S) -> Styled<Self, S> {
        Styled { shape: self, style }
    }
}
