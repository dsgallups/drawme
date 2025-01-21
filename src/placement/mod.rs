mod axes;
pub use axes::*;

mod direction;
pub use direction::*;

mod relative;
pub use relative::*;

mod bbox;
pub use bbox::*;

use nalgebra::{Point2, Rotation2, Scalar, Vector2};

//pub type Point = Point2<f64>;
//pub type Vector = Vector2<f64>;
//pub type Rotation = Rotation2<f64>;
//pub type Isometry = Isometry2<f64>;
//pub type Similarity = Similarity2<f64>;

pub trait PointExt<Unit: Scalar> {
    fn rotate_around(&self, rotation: &Rotation2<Unit>, pivot: &Point2<Unit>) -> Point2<Unit>;
}

impl<Unit: Scalar> PointExt<Unit> for Point2<Unit> {
    fn rotate_around(&self, rotation: &Rotation2<Unit>, pivot: &Point2<Unit>) -> Point2<Unit> {
        let translated_point = self - pivot;

        let rotated_point = rotation * translated_point;

        Point2::from(rotated_point + pivot.coords)
    }
}

pub trait IntoPoint<Unit: Scalar> {
    fn into_point(self) -> Point2<Unit>;
}

impl<Unit: Scalar> IntoPoint<Unit> for (Unit, Unit) {
    fn into_point(self) -> Point2<Unit> {
        Point2::new(self.0, self.1)
    }
}
impl<Unit: Scalar> IntoPoint<Unit> for Point2<Unit> {
    fn into_point(self) -> Point2<Unit> {
        self
    }
}

pub trait IntoVector<Unit> {
    fn into_vector(self) -> Vector2<Unit>;
}

impl<Unit> IntoVector<Unit> for (Unit, Unit) {
    fn into_vector(self) -> Vector2<Unit> {
        Vector2::new(self.0, self.1)
    }
}
impl<Unit> IntoVector<Unit> for Vector2<Unit> {
    fn into_vector(self) -> Vector2<Unit> {
        self
    }
}
