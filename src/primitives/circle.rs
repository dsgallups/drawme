use nalgebra::{Point2, Scalar};

use crate::prelude::*;

/// A simple circle.
#[derive(Debug, Clone, Copy)]
pub struct Circle<Unit: Scalar = f64> {
    /// The position of the center of the circle
    pub position: Point2<Unit>,
    /// The radius of the circle
    pub radius: Unit,
}

impl<Unit: Scalar> Circle<Unit> {
    pub fn new(position: impl IntoPoint<Unit>, radius: f64) -> Self {
        Self {
            position: position.into_point(),
            radius,
        }
    }
}

impl<Unit: Scalar + Clone> Primitive for Circle<Unit> {
    type Unit = Unit;
    fn draw_primitive<'c, C, S>(&'c self, canvas: &'c mut C) -> impl FnMut(S) + 'c
    where
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle,
    {
        |style| {
            canvas.circle(style, self.position.clone(), self.radius.clone());
        }
    }
}

// fn do_half<'style, 'cvs, 'cir, C: Canvas + ?Sized>(
//     circle: &'cir Circle,
//     canvas: &'cvs mut C,
// ) -> impl FnOnce(DrawStyle<'style>) + use<'cvs, 'style, 'cir, C> {
//     return |style: DrawStyle<'style>| canvas.circle(style, circle.position, circle.radius);
// }
