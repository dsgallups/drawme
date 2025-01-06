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
    pub fn new(position: impl Into<Point>, radius: f64) -> Self {
        Self {
            position: position.into(),
            radius,
        }
    }
}

impl<C: Canvas + ?Sized> Drawable<C> for Circle {
    fn draw(&self, canvas: &mut C) {
        canvas.circle(self.position, self.radius)
    }
}
