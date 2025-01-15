use std::fmt;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Dbg<T> {
    pub inner: T,
    pub logs: Vec<String>,
}

impl<T> Dbg<T> {
    pub fn log(&mut self, val: impl fmt::Display) {
        self.logs.push(val.to_string());
        println!("log {}: {}", self.logs.len(), val);
    }
}

impl<T: Canvas> Canvas for Dbg<T> {
    fn set_fill(&mut self, fill: Option<&Paint>) {
        self.log(format!("set_fill: {:?}", fill));
        self.inner.set_fill(fill);
    }
    fn set_stroke_color(&mut self, paint: Option<&Paint>) {
        self.log(format!("set_stroke_color: {:?}", paint));
        self.inner.set_stroke_color(paint);
    }
    fn set_stroke_width(&mut self, width: Option<f64>) {
        self.log(format!("set_stroke_width: {:?}", width));
        self.inner.set_stroke_width(width);
    }

    fn path(&mut self, path: &Path) {
        self.log(format!("path: {:?}", path));
        self.inner.path(path);
    }
    fn text(&mut self, text: &str, font: &FontProps<'_>) {
        self.log(format!("text: {:?}, {:?}", text, font));
        self.inner.text(text, font);
    }
    fn image(&mut self, src: &ImageSource) {
        self.log(format!("image: {:?}", src));
        self.inner.image(src);
    }
    fn circle(&mut self, point: Point, radius: f64) {
        self.log(format!("circle: {:?}, {:?}", point, radius));
        self.inner.circle(point, radius);
    }
    fn rectangle(&mut self, top_left: Point, bottom_right: Point) {
        self.log(format!("rectangle: {:?}, {:?}", top_left, bottom_right));
        self.inner.rectangle(top_left, bottom_right);
    }
}
