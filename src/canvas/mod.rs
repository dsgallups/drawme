use crate::prelude::*;

pub trait Canvas {
    fn set_fill(&mut self, fill: &Fill<'_>);
    fn set_stroke_color(&mut self, fill: &Fill<'_>);
    fn set_stroke_width(&mut self, fill: f64);

    fn path(&mut self, path: ());
    fn text(&mut self, text: ());
    fn rectangle(&mut self, top_left: Point, bottom_right: Point);
    fn circle(&mut self, point: Point, radius: f64);
    fn image(&mut self, src: ImageSource);
}
