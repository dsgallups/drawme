use std::fmt::Debug;

use nalgebra::{Point2, Rotation2, Scalar, Vector2};

use crate::prelude::*;

/// Rectangle is a simple rectangle
///
/// Its operations are non-commutative.
///
/// TODO: check if translation occurs before rotation
#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle<Unit: Scalar = f64> {
    /// The point closest to the origin (top_left)
    top_left: Point2<Unit>,
    // The point farthest from the origin (bottom_right)
    bottom_right: Point2<Unit>,
    /// The rotation of the rectangle
    rot: Rotation2<Unit>,
}

impl<Unit: Scalar> Rectangle<Unit> {
    pub fn new(closest: impl IntoPoint<Unit>, farthest: impl IntoPoint<Unit>) -> Self {
        Self {
            top_left: closest.into_point(),
            bottom_right: farthest.into_point(),
            rot: Rotation2::identity(),
        }
    }

    pub fn zero() -> Self {
        Self {
            top_left: Point2::origin(),
            bottom_right: Point2::origin(),
            rot: Rotation2::identity(),
        }
    }

    pub const fn new_from_raw(
        closest: Point2<Unit>,
        farthest: Point2<Unit>,
        rot: Rotation2<Unit>,
    ) -> Self {
        Self {
            top_left: closest,
            bottom_right: farthest,
            rot,
        }
    }

    pub fn with_rotation(mut self, rot: Rotation2<Unit>) -> Self {
        self.rot = rot;
        self
    }

    /// Rotates around its center. Rectangle does not implement transformation since it cannot be rotated around a point.
    pub fn rotate(&mut self, rot: Rotation2<Unit>) {
        self.rot = rot;
    }

    /// Set the closest x coordinate
    pub fn set_top_left_x(&mut self, x: Unit) {
        self.top_left.x = x;
    }

    /// Set the closest y coordinate
    pub fn set_top_left_y(&mut self, y: Unit) {
        self.top_left.y = y;
    }

    /// Set the bottom left x coordinate
    pub fn set_bottom_right_x(&mut self, x: Unit) {
        self.bottom_right.x = x;
    }

    /// Set the bottom right y coordinate
    pub fn set_bottom_right_y(&mut self, y: Unit) {
        self.bottom_right.y = y;
    }

    pub fn closest_mut(&mut self) -> &mut Point2<Unit> {
        &mut self.top_left
    }

    pub fn farthest_mut(&mut self) -> &mut Point2<Unit> {
        &mut self.bottom_right
    }

    /// returns the rotation of the rectangle
    pub fn rotation(&self) -> Rotation2<Unit> {
        self.rot
    }

    /// returns closest point to origin accounting for rotation
    pub fn top_left(&self) -> Point2<Unit> {
        let mut init = self.top_left;
        if self.rot != Rotation2::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }

        init
    }

    /// This function may be incorrectly implemented if a rotation has occurred. Need to check.
    pub fn translate(&mut self, by: Vector2<Unit>) {
        self.top_left += by;
        self.bottom_right += by;
    }

    /// Returns the pixel that utilizes the bottom_right x coordinate and top_left y coordinate, accounting
    /// for rotation.
    pub fn top_right(&self) -> Point2<Unit> {
        let mut init = Point2::new(self.bottom_right.x, self.top_left.y);
        if self.rot != Rotation2::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }
        init
    }
    /// Returns the pixel that utilizes the top left x coordinate and bottom right y coordinate, accounting
    /// for rotation.
    pub fn bottom_left(&self) -> Point2<Unit> {
        let mut init = Point2::new(self.top_left.x, self.bottom_right.y);
        if self.rot != Rotation2::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }
        init
    }

    /// returns bottom right accounting for rotation
    pub fn bottom_right(&self) -> Point2<Unit> {
        let mut init = self.bottom_right;
        if self.rot != Rotation2::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }
        init
    }

    pub fn from_dimensions_and_center(dimensions: Vector2<Unit>, center: Point2<Unit>) -> Self {
        let half = dimensions.scale(0.5);

        let closest = center - half;

        let farthest = center + half;

        Self::new(closest, farthest)
    }

    /*    pub fn into_rounded_rectangle(self) -> RoundedRectangle<Unit>
    where
        Unit: Zero,
    {
        RoundedRectangle::from_rectangle(self)
    } */

    /// returns closest not accounting for rotation
    pub fn top_left_raw(&self) -> Point2<Unit> {
        self.top_left
    }

    /// returns top right not accounting for rotation
    pub fn top_right_raw(&self) -> Point2<Unit> {
        Point2::new(self.bottom_right.x, self.top_left.y)
    }
    /// returns bottom left not accounting for rotation
    pub fn bottom_left_raw(&self) -> Point2<Unit> {
        Point2::new(self.top_left.x, self.bottom_right.y)
    }

    /// returns bottom right not accounting for rotation
    pub fn bottom_right_raw(&self) -> Point2<Unit> {
        self.bottom_right
    }

    pub fn absolute_center(&self) -> Point2<Unit> {
        nalgebra::center(&self.top_left(), &self.bottom_right())
    }

    pub fn from_dimensions_and_offset(offset: Point2<Unit>, dimensions: Vector2<Unit>) -> Self {
        let closest = offset;
        let farthest = offset + dimensions;

        Self {
            top_left: closest,
            bottom_right: farthest,
            rot: Rotation2::identity(),
        }
    }
    pub fn from_dimensions(dimensions: Vector2<Unit>) -> Self {
        let farthest = Point2::new(dimensions.x, dimensions.y);
        Self {
            top_left: Point2::origin(),
            bottom_right: farthest,
            rot: Rotation2::identity(),
        }
    }
    pub fn with_offset(mut self, offset: impl IntoVector<Unit>) -> Self {
        let offset = offset.into_vector();
        self.top_left += offset;
        self.bottom_right += offset;
        self
    }

    pub fn relative_center(&self) -> Vector2<Unit> {
        Vector2::new(self.width() / 2., self.height() / 2.)
    }

    fn height(&self) -> Unit {
        self.bottom_right.y - self.top_left.y
    }

    fn width(&self) -> Unit {
        self.bottom_right.x - self.top_left.x
    }
    pub fn dimensions(&self) -> Vector2<Unit> {
        self.bottom_right - self.top_left
    }
}
/*impl<Unit: DrawableUnit> DrawableCommand for Rectangle<Unit> {
    type DrawUnit = Unit;
    fn into_draw_command(self) -> DrawCommand<Self::DrawUnit> {
        DrawCommand::path([
            PathCommand::move_to(self.top_left()),
            PathCommand::line_to(self.top_right()),
            PathCommand::line_to(self.bottom_right()),
            PathCommand::line_to(self.bottom_left()),
            PathCommand::line_to(self.top_left()),
        ])
    }
}*/
