use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Paint {
    Solid(SolidColor),
    Gradient(Gradient),
}
