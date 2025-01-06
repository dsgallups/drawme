use crate::prelude::*;

/// A simple circle.
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    /// The position of the center of the circle
    pub position: Vec2,
    /// The radius of the circle
    pub radius: f64,
}
