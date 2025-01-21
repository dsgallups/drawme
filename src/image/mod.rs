use crate::prelude::*;
use nalgebra::{Rotation2, Vector2};
#[cfg(feature = "serde")]
use serde::Serialize;
use url::Url;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ImageSource(Url);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImageProps<Unit = f64> {
    offset: Vector2<Unit>,
    image_width: Option<Unit>,
    image_height: Option<Unit>,
    rotation: Option<Rotation2<Unit>>,
}

impl<Unit> ImageProps<Unit> {
    pub fn new(
        offset: impl IntoVector<Unit>,
        image_width_override: Option<Unit>,
        image_height_override: Option<Unit>,
        rotation: Option<Rotation2<Unit>>,
    ) -> Self {
        Self {
            offset: offset.into(),
            image_width: image_width_override,
            image_height: image_height_override,
            rotation,
        }
    }

    pub fn empty() -> Self {
        Self {
            offset: Vector2::zeroes(),
            image_width: None,
            image_height: None,
            rotation: None,
        }
    }

    pub fn offset(&self) -> Vector2<Unit> {
        self.offset
    }
    pub fn offset_mut(&mut self) -> &mut Vector2<Unit> {
        &mut self.offset
    }

    pub fn rotation_mut(&mut self) -> Option<&mut Rotation2<Unit>> {
        self.rotation.as_mut()
    }

    pub fn image_width(&self) -> Option<Unit> {
        self.image_width
    }

    pub fn image_height(&self) -> Option<Unit> {
        self.image_height
    }

    pub fn set_rotation(&mut self, rotation: Rotation2<Unit>) -> &mut Self {
        self.rotation = Some(rotation);
        self
    }
}
