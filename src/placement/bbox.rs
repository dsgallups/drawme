use nalgebra::{Scalar, Vector2};
use num_traits::Zero;

use crate::prelude::DrawUnit;

use super::IntoVector;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BoundingBox<Unit: Scalar = f64> {
    offset: Vector2<Unit>,
    dimensions: Vector2<Unit>,
}

impl<Unit: Scalar> BoundingBox<Unit> {
    /// Creates a bounding box an offset away from the origin with the given dimensions
    pub fn new_with_offset(
        top_left: impl IntoVector<Unit>,
        dimensions: impl IntoVector<Unit>,
    ) -> Self {
        Self {
            offset: top_left.into_vector(),
            dimensions: dimensions.into_vector(),
        }
    }

    pub fn set_bounding_width(&mut self, width: Unit) -> &mut Self {
        self.dimensions.x = width;
        self
    }
    pub fn set_bounding_height(&mut self, height: Unit) -> &mut Self {
        self.dimensions.y = height;
        self
    }

    pub fn set_dimensions(&mut self, dimensions: impl IntoVector<Unit>) -> &mut Self {
        self.dimensions = dimensions.into_vector();
        self
    }

    pub fn set_offset(&mut self, offset: impl IntoVector<Unit>) -> &mut Self {
        self.offset = offset.into_vector();
        self
    }

    pub fn offset_mut(&mut self) -> &mut Vector2<Unit> {
        &mut self.offset
    }

    pub fn set_width(&mut self, width: Unit) -> &mut Self {
        self.dimensions.x = width;
        self
    }
    pub fn set_height(&mut self, height: Unit) -> &mut Self {
        self.dimensions.y = height;
        self
    }
}

impl<Unit: Scalar + Zero> BoundingBox<Unit> {
    pub fn new(dimensions: impl IntoVector<Unit>) -> Self {
        Self {
            offset: Vector2::zeros(),
            dimensions: dimensions.into_vector(),
        }
    }
    pub fn zero() -> Self {
        Self {
            offset: Vector2::zeros(),
            dimensions: Vector2::zeros(),
        }
    }
}

impl<Unit: Scalar + Copy> BoundingBox<Unit> {
    pub fn offset(&self) -> Vector2<Unit> {
        self.offset
    }

    pub fn dimensions(&self) -> Vector2<Unit> {
        self.dimensions
    }
}

impl<Unit: DrawUnit> BoundingBox<Unit> {
    pub fn center(&self) -> Vector2<Unit> {
        let center_x = self.offset.x + (self.dimensions.x / Unit::TWO);
        let center_y = self.offset.y + (self.dimensions.y / Unit::TWO);

        (center_x, center_y).into_vector()
    }

    // pub fn position_inside(self, position: RelativePosition<Unit>) -> InBoundingBox<Unit> {
    //     InBoundingBox {
    //         bounding_box: self,
    //         position,
    //     }
    // }
}
