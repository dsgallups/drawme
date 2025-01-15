use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub};

use crate::prelude::*;

/// Rectangle is a simple rectangle
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
    pub fn new(closest: impl Into<Point>, farthest: impl Into<Point>) -> Self {
        Self {
            top_left: closest.into(),
            bottom_right: farthest.into(),
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

    pub fn new_from_raw(closest: Point, farthest: Point, rot: Rotation) -> Self {
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

    /// returns the width of the rectangle
    pub fn rotation(&self) -> &Rotation {
        let v = self.rot;
        &self.rot
    }

    /// returns closest point to origin accounting for rotation
    pub fn top_left(&self) -> Point {
        let mut init = self.top_left;
        if self.rot != Rotation::identity() {
            init = init.rotate_around(self.rot, self.absolute_center());
        }

        init
    }

    pub fn translate(&mut self, point: Vector) {
        let v = self.top_left + point;
        self.top_left = self.top_left + point.clone();
        self.bottom_right += point;
    }

    /// Returns the pixel that utilizes the bottom_right x coordinate and top_left y coordinate, accounting
    /// for rotation.
    pub fn top_right(&self) -> Point {
        let mut init = Point::new(self.bottom_right.x, self.top_left.y);
        if self.rot != Rotation::identity() {
            init = init.rotate_around(self.rot, self.absolute_center());
        }
        init
    }
    /// Returns the pixel that utilizes the top left x coordinate and bottom right y coordinate, accounting
    /// for rotation.
    pub fn bottom_left(&self) -> Point {
        let mut init = Point::new(self.top_left.x, self.bottom_right.y);
        if self.rot != Rotation::identity() {
            init = init.rotate_around(self.rot, self.absolute_center());
        }
        init
    }

    /// returns bottom right accounting for rotation
    pub fn bottom_right(&self) -> Point {
        let mut init = self.bottom_right;
        if self.rot != Rotation::identity() {
            init = init.rotate_around(self.rot, self.absolute_center());
        }
        init
    }

    pub fn from_dimensions_and_center(
        dimensions: impl Into<Vector>,
        center: impl Into<Vector>,
    ) -> Self {
        let dimensions: Vector = dimensions.into();
        let center: Vector = center.into();

        let closest = Point::new(
            center.x() - dimensions.x() / 2.,
            center.y() - dimensions.y() / 2.,
        );
        let farthest = Vector::new(
            center.x.clone() + dimensions.width / 2.,
            center.y.clone() + dimensions.height / 2.,
        );
        Self {
            top_left: closest,
            bottom_right: farthest,
            rot: Rotation::zero(),
        }
    }

    /*    pub fn into_rounded_rectangle(self) -> RoundedRectangle<Unit>
    where
        Unit: Zero,
    {
        RoundedRectangle::from_rectangle(self)
    } */

    /// returns closest not accounting for rotation
    pub fn top_left_raw(&self) -> Vector {
        self.top_left
    }

    /// returns top right not accounting for rotation
    pub fn top_right_raw(&self) -> Vector {
        Vector::new(self.bottom_right.x, self.top_left.y)
    }
    /// returns bottom left not accounting for rotation
    pub fn bottom_left_raw(&self) -> Vector {
        Vector::new(self.top_left.x, self.bottom_right.y)
    }

    /// returns bottom right not accounting for rotation
    pub fn bottom_right_raw(&self) -> &Vector {
        &self.bottom_right
    }

    pub fn absolute_center(&self) -> Point {
        let center = nalgebra::center(&self.top_left(), &self.bottom_right());
        let top_left = self.top_left();
        let dims = self.dimensions();
        // Vector::new(
        //     top_left.x.clone() + (self.width().clone() / 2.),
        //     top_left.y.clone() + (self.height().clone() / 2.),
        // )
        todo!()
    }

    pub fn from_dimensions_and_offset(offset: impl Into<Point>, dimensions: Vector) -> Self {
        let closest = offset.into();
        let farthest = (
            closest.x.clone() + dimensions.width,
            closest.y.clone() + dimensions.height,
        )
            .into();
        Self {
            top_left: closest,
            bottom_right: farthest,
            rot: Rotation::zero(),
        }
    }
    pub fn from_dimensions(dimensions: Vector) -> Self {
        let farthest = (dimensions.x(), dimensions.y()).into();
        Self {
            top_left: Point::zero(),
            bottom_right: farthest,
            rot: Rotation::zero(),
        }
    }
    pub fn with_offset(mut self, offset: Vector) -> Self {
        self.top_left += offset.clone();
        self.bottom_right += offset;
        self
    }

    pub const fn const_new(closest: Vector, farthest: Vector) -> Self {
        Self {
            top_left: closest,
            bottom_right: farthest,
            rot: Rotation::zero(),
        }
    }

    pub fn relative_center(&self) -> Vector {
        Vector::new(self.width() / 2., self.height() / 2.)
    }

    fn height(&self) -> f64 {
        self.bottom_right.y() - self.top_left.y()
    }

    fn width(&self) -> f64 {
        self.bottom_right.x() - self.top_left.x()
    }
    fn dimensions(&self) -> Vector {
        Vector::new(self.width(), self.height())
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
