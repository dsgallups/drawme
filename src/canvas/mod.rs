use crate::prelude::*;

#[cfg(feature = "svg")]
pub mod svg;

pub trait Canvas {
    fn set_fill(&mut self, paint: &Paint);
    fn set_stroke_color(&mut self, paint: &Paint);
    fn set_stroke_width(&mut self, width: f64);

    fn path(&mut self, path: &Path);
    fn text(&mut self, text: &str, font: &FontProps<'_>);
    fn rectangle(&mut self, top_left: Point, bottom_right: Point) {
        let mut path = Path::with_capacity(5);
        path.move_to(top_left);
        path.line_to((bottom_right.x(), top_left.y()));
        path.line_to(bottom_right);
        path.line_to((top_left.x(), bottom_right.y()));
        path.line_to(top_left);

        self.path(&path);
    }
    fn circle(&mut self, point: Point, radius: f64);
    fn image(&mut self, src: ImageSource);
}
