use crate::prelude::*;

/// A simple circle.
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    /// The position of the center of the circle
    pub position: Point,
    /// The radius of the circle
    pub radius: f64,
}

impl Circle {
    pub fn new(position: impl IntoPoint, radius: f64) -> Self {
        Self {
            position: position.into_point(),
            radius,
        }
    }
}

impl Primitive for Circle {
    fn draw_primitive<'c, C>(&'c self, canvas: &'c mut C) -> impl FnMut(DrawStyle<'_>) + 'c
    where
        C: Canvas,
    {
        |style: DrawStyle<'_>| {
            canvas.circle(style, self.position, self.radius);
        }
    }
    fn draw_primitive_boxed<'c>(
        &'c self,
        canvas: &'c mut dyn Canvas,
    ) -> Box<dyn FnMut(DrawStyle<'_>) + 'c> {
        Box::new(|style: DrawStyle<'_>| {
            canvas.circle(style, self.position, self.radius);
        })
    }
}

// fn do_half<'style, 'cvs, 'cir, C: Canvas + ?Sized>(
//     circle: &'cir Circle,
//     canvas: &'cvs mut C,
// ) -> impl FnOnce(DrawStyle<'style>) + use<'cvs, 'style, 'cir, C> {
//     return |style: DrawStyle<'style>| canvas.circle(style, circle.position, circle.radius);
// }
