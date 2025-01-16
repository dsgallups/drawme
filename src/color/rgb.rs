#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Rgb {
    /// red
    pub r: u8,
    /// green
    pub g: u8,
    /// blue
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn css(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub const fn into_tuple(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}
