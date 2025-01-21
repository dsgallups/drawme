use std::fmt;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Dbg<T: ?Sized> {
    pub logs: Vec<String>,
    pub inner: T,
}

impl<T> Dbg<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
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
    fn path<S: AsDrawStyle>(&mut self, style: S, path: &Path) {
        self.log(format!(
            "style: {:?}, path: {:?}",
            DrawStyle::from_style_ref(&style),
            path
        ));
        self.inner.path(style, path);
    }
    fn text<S: AsDrawStyle>(&mut self, style: S, text: &str, font: &FontProps<'_>, iso: Isometry) {
        self.log(format!(
            "style: {:?}, text: {:?}, {:?}, {:?}",
            DrawStyle::from_style_ref(&style),
            text,
            font,
            iso
        ));
        self.inner.text(style, text, font, iso);
    }
    fn image(&mut self, src: &ImageSource) {
        self.log(format!("image: {:?}", src));
        self.inner.image(src);
    }
    fn circle<S: AsDrawStyle>(&mut self, style: S, point: Point, radius: f64) {
        self.log(format!(
            "style: {:?}, circle: {:?}, {:?}",
            DrawStyle::from_style_ref(&style),
            point,
            radius
        ));
        self.inner.circle(style, point, radius);
    }
    fn rectangle<S: AsDrawStyle>(&mut self, style: S, rectangle: &Rectangle) {
        self.log(format!(
            "style: {:?}, rectangle: {:?}",
            DrawStyle::from_style_ref(&style),
            rectangle
        ));
        self.inner.rectangle(style, rectangle);
    }
}
