use nalgebra::{Point2, Rotation2, Vector2};

pub type Point = Point2<f64>;
pub type Vector = Vector2<f64>;
pub type Rotation = Rotation2<f64>;

pub trait PointExt {
    fn rotate_around(&self, rotation: Rotation, pivot: Point) -> Point;
}

impl PointExt for Point {
    fn rotate_around(&self, rotation: Rotation, pivot: Point) -> Point {
        let translated_point = self - pivot;

        let rotated_point = rotation * translated_point;

        Point::from(rotated_point + pivot.coords)
    }
}
