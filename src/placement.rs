use nalgebra::{Isometry2, Point2, Rotation2, Similarity2, Vector2};

pub type Point = Point2<f64>;
pub type Vector = Vector2<f64>;
pub type Rotation = Rotation2<f64>;
pub type Isometry = Isometry2<f64>;
pub type Similarity = Similarity2<f64>;

pub trait PointExt {
    fn rotate_around(&self, rotation: &Rotation, pivot: &Point) -> Point;
}

impl PointExt for Point {
    fn rotate_around(&self, rotation: &Rotation, pivot: &Point) -> Point {
        let translated_point = self - pivot;

        let rotated_point = rotation * translated_point;

        Point::from(rotated_point + pivot.coords)
    }
}

pub trait IntoPoint {
    fn into_point(self) -> Point;
}

impl IntoPoint for (f64, f64) {
    fn into_point(self) -> Point {
        Point::new(self.0, self.1)
    }
}
impl IntoPoint for Point {
    fn into_point(self) -> Point {
        self
    }
}

pub trait IntoVector {
    fn into_vector(self) -> Vector;
}

impl IntoVector for (f64, f64) {
    fn into_vector(self) -> Vector {
        Vector::new(self.0, self.1)
    }
}
impl IntoVector for Vector {
    fn into_vector(self) -> Vector {
        self
    }
}
