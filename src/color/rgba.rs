#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Rgba {
    /// red
    r: u8,
    /// green
    g: u8,
    /// blue
    b: u8,
    /// alpha (opacity betwen 0. and 1.)
    a: f64,
}
