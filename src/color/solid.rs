use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SolidColor {
    Opaque(Rgb),
    Alpha(Rgba),
}
impl SolidColor {
    pub fn css(&self) -> String {
        match self {
            SolidColor::Alpha(a) => a.css(),
            SolidColor::Opaque(o) => o.css(),
        }
    }
}
