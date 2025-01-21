use std::borrow::Cow;

use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Paint<'a> {
    Solid(SolidColor),
    Gradient(Cow<'a, Gradient>),
}

impl Paint<'_> {
    pub const fn solid(solid_color: SolidColor) -> Self {
        Self::Solid(solid_color)
    }

    //clones solid color, weakly clones gradient, even if owned
    pub const fn clone_shallow(&self) -> Paint<'_> {
        match self {
            Paint::Solid(s) => Paint::Solid(*s),
            Paint::Gradient(Cow::Borrowed(g)) => Paint::Gradient(Cow::Borrowed(g)),
            Paint::Gradient(Cow::Owned(g)) => Paint::Gradient(Cow::Borrowed(g)),
        }
    }
}

impl From<SolidColor> for Paint<'_> {
    fn from(value: SolidColor) -> Self {
        Self::Solid(value)
    }
}

impl From<Gradient> for Paint<'_> {
    fn from(value: Gradient) -> Self {
        Paint::Gradient(Cow::Owned(value))
    }
}

impl<'a> From<&'a Gradient> for Paint<'a> {
    fn from(value: &'a Gradient) -> Self {
        Paint::Gradient(Cow::Borrowed(value))
    }
}

impl From<Rgb> for Paint<'_> {
    fn from(value: Rgb) -> Self {
        Paint::Solid(SolidColor::Opaque(value))
    }
}

impl From<Rgba> for Paint<'_> {
    fn from(value: Rgba) -> Self {
        Paint::Solid(SolidColor::Alpha(value))
    }
}

/*
Comments remain when switching between risks/incidents

DONE Modal switcher underlying component doesn't resize when letting go of resizer

should accept padded zeroes on url
*/
