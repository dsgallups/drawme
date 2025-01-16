#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Rgba {
    /// red
    pub r: u8,
    /// green
    pub g: u8,
    /// blue
    pub b: u8,
    /// alpha (opacity betwen 0. and 1.)
    pub a: f64,
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn css(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }

    pub const fn into_tuple(self) -> (u8, u8, u8, f64) {
        (self.r, self.g, self.b, self.a)
    }
}
