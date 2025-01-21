use crate::prelude::*;

#[cfg(feature = "debug")]
pub mod debug;
#[cfg(feature = "svg")]
pub mod svg;
#[cfg(feature = "xml")]
pub mod xml;

pub trait Canvas {
    fn path<S: AsDrawStyle>(&mut self, style: S, path: &Path);
    fn text<S: AsDrawStyle>(
        &mut self,
        style: S,
        text: &str,
        font: &FontProps<'_>,
        isometry: Isometry,
    );
    fn rectangle<S: AsDrawStyle>(&mut self, style: S, rectangle: &Rectangle) {
        let top_left = rectangle.top_left();
        let bottom_right = rectangle.bottom_left();
        let mut path = Path::with_capacity(5);
        path.move_to(top_left);
        path.line_to(Point::new(bottom_right.x, top_left.y));
        path.line_to(bottom_right);
        path.line_to(Point::new(top_left.x, bottom_right.y));
        path.line_to(top_left);

        self.path(style, &path);
    }
    fn circle<S: AsDrawStyle>(&mut self, style: S, point: Point, radius: f64);
    fn image(&mut self, src: &ImageSource);
}
