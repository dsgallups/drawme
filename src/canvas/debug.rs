use std::fmt;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Dbg<T: ?Sized> {
    pub logs: Vec<String>,
    pub inner: T,
}

impl<T: ?Sized> Dbg<T> {
    pub fn log(&mut self, val: impl fmt::Display) {
        self.logs.push(val.to_string());
        println!("log {}: {}", self.logs.len(), val);
    }
}

impl<T> Canvas for Dbg<T>
where
    T: Canvas + ?Sized,
{
    fn path(&mut self, style: DrawStyle<'_>, path: &Path) {
        self.log(format!("style: {:?}, path: {:?}", style, path));
        self.inner.path(style, path);
    }
    fn text(&mut self, style: DrawStyle<'_>, text: &str, font: &FontProps<'_>, iso: Isometry) {
        self.log(format!(
            "style: {:?}, text: {:?}, {:?}, {:?}",
            style, text, font, iso
        ));
        self.inner.text(style, text, font, iso);
    }
    fn image(&mut self, src: &ImageSource) {
        self.log(format!("image: {:?}", src));
        self.inner.image(src);
    }
    fn circle(&mut self, style: DrawStyle<'_>, point: Point, radius: f64) {
        self.log(format!(
            "style: {:?}, circle: {:?}, {:?}",
            style, point, radius
        ));
        self.inner.circle(style, point, radius);
    }
    fn rectangle(&mut self, style: DrawStyle<'_>, rectangle: &Rectangle) {
        self.log(format!("style: {:?}, rectangle: {:?}", style, rectangle));
        self.inner.rectangle(style, rectangle);
    }
}
