use nalgebra::Vector2;
use num_traits::Num;

use super::IntoVector;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoundingBox<Unit = f64> {
    offset: Vector2<Unit>,
    dimensions: Vector2<Unit>,
}

impl<Unit> BoundingBox<Unit> {
    pub fn new(dimensions: impl IntoVector<Unit>) -> Self {
        Self {
            offset: Vector2::zeros(),
            dimensions: dimensions.into_vector(),
        }
    }

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

    pub fn offset(&self) -> Vector2<Unit> {
        &self.offset
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

    pub fn center(&self) -> Vector2<Unit>
    where
        Unit: Num + Clone,
    {
        let center_x = self.top_left.x.clone() + (self.dimensions.width.clone() / Unit::two());
        let center_y = self.top_left.y.clone() + (self.dimensions.height.clone() / Unit::two());

        (center_x, center_y).into_vector()
    }

    pub fn dimensions(&self) -> Vector2<Unit> {
        self.dimensions
    }

    // pub fn position_inside(self, position: RelativePosition<Unit>) -> InBoundingBox<Unit> {
    //     InBoundingBox {
    //         bounding_box: self,
    //         position,
    //     }
    // }

    pub fn set_offset(&mut self, offset: impl IntoVector<Unit>) -> &mut Self {
        self.offset = offset.into_vector();
        self
    }

    pub fn offset_mut(&mut self) -> &mut Vector2<Unit> {
        &mut self.offset
    }

    pub fn zero() -> Self {
        Self {
            offset: Vector2::zero(),
            dimensions: Vector2::zero(),
        }
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
