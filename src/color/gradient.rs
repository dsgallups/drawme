use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "transition"))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Gradient {
    Linear {
        rot: Rotation,
        colors: Vec<(SolidColor, f64)>,
    },
    Radial {
        center: Point,
        colors: Vec<(SolidColor, f64)>,
    },
}
