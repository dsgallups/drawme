use std::fmt::Debug;

use crate::prelude::*;

/// Rectangle is a simple rectangle
///
/// Its operations are non-commutative.
///
/// TODO: check if translation occurs before rotation
#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle {
    /// The point closest to the origin (top_left)
    top_left: Point,
    // The point farthest from the origin (bottom_right)
    bottom_right: Point,
    /// The rotation of the rectangle
    rot: Rotation,
}

impl Rectangle {
    pub fn new(closest: impl IntoPoint, farthest: impl IntoPoint) -> Self {
        Self {
            top_left: closest.into_point(),
            bottom_right: farthest.into_point(),
            rot: Rotation::identity(),
        }
    }

    pub fn zero() -> Self {
        Self {
            top_left: Point::origin(),
            bottom_right: Point::origin(),
            rot: Rotation::identity(),
        }
    }

    pub const fn new_from_raw(closest: Point, farthest: Point, rot: Rotation) -> Self {
        Self {
            top_left: closest,
            bottom_right: farthest,
            rot,
        }
    }

    pub fn with_rotation(mut self, rot: Rotation) -> Self {
        self.rot = rot;
        self
    }

    /// Rotates around its center. Rectangle does not implement transformation since it cannot be rotated around a point.
    pub fn rotate(&mut self, rot: Rotation) {
        self.rot = rot;
    }

    /// Set the closest x coordinate
    pub fn set_top_left_x(&mut self, x: f64) {
        self.top_left.x = x;
    }

    /// Set the closest y coordinate
    pub fn set_top_left_y(&mut self, y: f64) {
        self.top_left.y = y;
    }

    /// Set the bottom left x coordinate
    pub fn set_bottom_right_x(&mut self, x: f64) {
        self.bottom_right.x = x;
    }

    /// Set the bottom right y coordinate
    pub fn set_bottom_right_y(&mut self, y: f64) {
        self.bottom_right.y = y;
    }

    pub fn closest_mut(&mut self) -> &mut Point {
        &mut self.top_left
    }

    pub fn farthest_mut(&mut self) -> &mut Point {
        &mut self.bottom_right
    }

    /// returns the rotation of the rectangle
    pub fn rotation(&self) -> Rotation {
        self.rot
    }

    /// returns closest point to origin accounting for rotation
    pub fn top_left(&self) -> Point {
        let mut init = self.top_left;
        if self.rot != Rotation::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }

        init
    }

    /// This function may be incorrectly implemented if a rotation has occurred. Need to check.
    pub fn translate(&mut self, by: Vector) {
        self.top_left += by;
        self.bottom_right += by;
    }

    /// Returns the pixel that utilizes the bottom_right x coordinate and top_left y coordinate, accounting
    /// for rotation.
    pub fn top_right(&self) -> Point {
        let mut init = Point::new(self.bottom_right.x, self.top_left.y);
        if self.rot != Rotation::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }
        init
    }
    /// Returns the pixel that utilizes the top left x coordinate and bottom right y coordinate, accounting
    /// for rotation.
    pub fn bottom_left(&self) -> Point {
        let mut init = Point::new(self.top_left.x, self.bottom_right.y);
        if self.rot != Rotation::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }
        init
    }

    /// returns bottom right accounting for rotation
    pub fn bottom_right(&self) -> Point {
        let mut init = self.bottom_right;
        if self.rot != Rotation::identity() {
            init = init.rotate_around(&self.rot, &self.absolute_center());
        }
        init
    }

    pub fn from_dimensions_and_center(dimensions: Vector, center: Point) -> Self {
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
    pub fn top_left_raw(&self) -> Point {
        self.top_left
    }

    /// returns top right not accounting for rotation
    pub fn top_right_raw(&self) -> Point {
        Point::new(self.bottom_right.x, self.top_left.y)
    }
    /// returns bottom left not accounting for rotation
    pub fn bottom_left_raw(&self) -> Point {
        Point::new(self.top_left.x, self.bottom_right.y)
    }

    /// returns bottom right not accounting for rotation
    pub fn bottom_right_raw(&self) -> Point {
        self.bottom_right
    }

    pub fn absolute_center(&self) -> Point {
        nalgebra::center(&self.top_left(), &self.bottom_right())
    }

    pub fn from_dimensions_and_offset(offset: Point, dimensions: Vector) -> Self {
        let closest = offset;
        let farthest = offset + dimensions;

        Self {
            top_left: closest,
            bottom_right: farthest,
            rot: Rotation::identity(),
        }
    }
    pub fn from_dimensions(dimensions: Vector) -> Self {
        let farthest = Point::new(dimensions.x, dimensions.y);
        Self {
            top_left: Point::origin(),
            bottom_right: farthest,
            rot: Rotation::identity(),
        }
    }
    pub fn with_offset(mut self, offset: impl IntoVector) -> Self {
        let offset = offset.into_vector();
        self.top_left += offset;
        self.bottom_right += offset;
        self
    }

    pub fn relative_center(&self) -> Vector {
        Vector::new(self.width() / 2., self.height() / 2.)
    }

    fn height(&self) -> f64 {
        self.bottom_right.y - self.top_left.y
    }

    fn width(&self) -> f64 {
        self.bottom_right.x - self.top_left.x
    }
    fn dimensions(&self) -> Vector {
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
