use crate::{impl_from, prelude::*};

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Paint {
    Solid(SolidColor),
    Gradient(Gradient),
}

impl Paint {
    pub fn solid(solid_color: SolidColor) -> Self {
        Self::Solid(solid_color)
    }
}

impl_from!(SolidColor, Paint, (color) => { Paint::solid(color) });
impl_from!(Gradient, Paint, (gradient) => { Paint::Gradient(gradient) });
impl_from!(Rgb, Paint, (rgb) => { Paint::Solid(SolidColor::Opaque(rgb)) });
impl_from!(Rgba, Paint, (rgba) => { Paint::Solid(SolidColor::Alpha(rgba)) });
