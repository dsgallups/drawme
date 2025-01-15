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

    pub fn css(&self) -> String {
        match self {
            Paint::Solid(s) => match s {
                SolidColor::Alpha(a) => format!("rgba({}, {}, {}, {})", a.r, a.g, a.b, a.a),
                SolidColor::Opaque(o) => format!("rgb({}, {}, {})", o.r, o.g, o.b),
            },
            Paint::Gradient(g) => todo!(),
        }
    }
}

impl_from!(SolidColor, Paint, (color) => { Paint::solid(color) });
impl_from!(Gradient, Paint, (gradient) => { Paint::Gradient(gradient) });
impl_from!(Rgb, Paint, (rgb) => { Paint::Solid(SolidColor::Opaque(rgb)) });
impl_from!(Rgba, Paint, (rgba) => { Paint::Solid(SolidColor::Alpha(rgba)) });
