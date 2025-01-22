use std::borrow::Cow;

use crate::prelude::*;

use nalgebra::Scalar;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Paint<'a, Unit: Scalar = f64> {
    Solid(SolidColor),
    Gradient(Cow<'a, Gradient<Unit>>),
}

impl Paint<'_> {
    pub const fn solid(solid_color: SolidColor) -> Self {
        Self::Solid(solid_color)
    }
}

impl<Unit: Scalar> Paint<'_, Unit> {
    //clones solid color, weakly clones gradient, even if owned
    pub const fn clone_shallow(&self) -> Paint<'_, Unit> {
        match self {
            Paint::Solid(s) => Paint::Solid(*s),
            Paint::Gradient(Cow::Borrowed(g)) => Paint::Gradient(Cow::Borrowed(g)),
            Paint::Gradient(Cow::Owned(g)) => Paint::Gradient(Cow::Borrowed(g)),
        }
    }

    /// If this paint is a borrowed gradient, the gradient will be cloned.
    pub fn into_owned(self) -> Paint<'static, Unit> {
        match self {
            Paint::Solid(s) => Paint::Solid(s),
            Paint::Gradient(Cow::Borrowed(g)) => Paint::Gradient(Cow::Owned(g.clone())),
            Paint::Gradient(Cow::Owned(g)) => Paint::Gradient(Cow::Owned(g)),
        }
    }
}

impl<U: Scalar> From<SolidColor> for Paint<'_, U> {
    fn from(value: SolidColor) -> Self {
        Self::Solid(value)
    }
}

impl<U: Scalar> From<Gradient<U>> for Paint<'_, U> {
    fn from(value: Gradient<U>) -> Self {
        Paint::Gradient(Cow::Owned(value))
    }
}

impl<'a, U: Scalar> From<&'a Gradient<U>> for Paint<'a, U> {
    fn from(value: &'a Gradient<U>) -> Self {
        Paint::Gradient(Cow::Borrowed(value))
    }
}

impl<U: Scalar> From<Rgb> for Paint<'_, U> {
    fn from(value: Rgb) -> Self {
        Paint::Solid(SolidColor::Opaque(value))
    }
}

impl<U: Scalar> From<Rgba> for Paint<'_, U> {
    fn from(value: Rgba) -> Self {
        Paint::Solid(SolidColor::Alpha(value))
    }
}

/*
Comments remain when switching between risks/incidents

DONE Modal switcher underlying component doesn't resize when letting go of resizer

should accept padded zeroes on url
*/
