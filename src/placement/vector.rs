use nalgebra::Vector2;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, PartialEq, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Vector(Vector2<f64>);
impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vector2::new(x, y))
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
}

impl From<(f64, f64)> for Vector {
    fn from(value: (f64, f64)) -> Self {
        Self::new(value.0, value.1)
    }
}
