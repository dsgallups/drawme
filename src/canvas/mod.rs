use crate::prelude::*;

pub mod debug;
#[cfg(feature = "svg")]
pub mod svg;
#[cfg(feature = "xml")]
pub mod xml;

pub trait Canvas {
    fn set_fill(&mut self, paint: Option<&Paint>);
    fn set_stroke_color(&mut self, paint: Option<&Paint>);
    fn set_stroke_width(&mut self, width: Option<f64>);

    fn path(&mut self, path: &Path);
    fn text(&mut self, text: &str, font: &FontProps<'_>);
    fn rectangle(&mut self, top_left: Point, bottom_right: Point) {
        let mut path = Path::with_capacity(5);
        path.move_to(top_left);
        path.line_to(Point::new(bottom_right.x, top_left.y));
        path.line_to(bottom_right);
        path.line_to(Point::new(top_left.x, bottom_right.y));
        path.line_to(top_left);

        self.path(&path);
    }
    fn circle(&mut self, point: Point, radius: f64);
    fn image(&mut self, src: &ImageSource);
}
