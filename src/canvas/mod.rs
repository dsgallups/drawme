use crate::prelude::*;

#[cfg(feature = "svg")]
pub mod svg;

pub trait Canvas {
    type Props;

    fn new_with_props(props: Self::Props) -> Self
    where
        Self: Sized;

    fn set_fill(&mut self, paint: &Paint);
    fn set_stroke_color(&mut self, paint: &Paint);
    fn set_stroke_width(&mut self, width: f64);

    fn path(&mut self, path: &Path);
    fn text(&mut self, text: &str, font: &FontProps<'_>);
    fn rectangle(&mut self, top_left: Point, bottom_right: Point);
    fn circle(&mut self, point: Point, radius: f64);
    fn image(&mut self, src: ImageSource);
}
