use crate::prelude::*;

#[doc = r#"
Something that can command a canvas to do something.

This is namely shapes and styles, and sometimes combinations of both.
"#]
pub trait Draw<C: Canvas + ?Sized> {
    fn draw(&self, canvas: &mut C);
}
