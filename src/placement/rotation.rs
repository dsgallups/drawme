use nalgebra::Rotation2;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Rotation(Rotation2<f64>);

impl Rotation {
    pub fn new(radian: f64) -> Self {
        Self(Rotation2::new(radian))
    }
}
