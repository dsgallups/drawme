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

impl<S> Primitive<S> for Circle {}

impl<C: Canvas + ?Sized> Draw<C> for Circle {
    fn draw(&self, canvas: &mut C) {
        canvas.circle(self.position, self.radius)
    }
}
