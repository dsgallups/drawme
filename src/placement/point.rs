use nalgebra::Point2;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Point(Point2<f64>);
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Point2::new(x, y))
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
}

impl From<(f64, f64)> for Point {
    fn from(value: (f64, f64)) -> Self {
        Self::new(value.0, value.1)
    }
}
