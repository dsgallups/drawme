/*
quick-xml v0.37.2 features:
       - arbitrary
       - async-tokio
       - document-features
       - encoding
       - encoding_rs
       - escape-html
       - overlapped-lists
       - serde
       - serde-types
       - serialize
       - tokio

*/

use crate::prelude::*;

mod xml;
pub use xml::*;

pub struct Svg<'a> {
    root: XmlNode<'a>,
    stroke_gradients: Vec<Gradient>,
    fill_gradients: Vec<Gradient>,
}

impl Default for Svg<'_> {
    fn default() -> Self {
        Self {
            root: XmlNode::builder("svg")
                .with_attributes([("xmlns", "http://w3.org/2000/svg")])
                .build(),
            stroke_gradients: vec![],
            fill_gradients: vec![],
        }
    }
}

impl Canvas for Svg<'_> {
    fn set_fill(&mut self, fill: &Paint) {
        todo!()
    }
    fn set_stroke_color(&mut self, paint: &Paint) {
        todo!()
    }
    fn set_stroke_width(&mut self, width: f64) {
        todo!()
    }

    fn path(&mut self, path: &Path) {
        todo!()
    }
    fn text(&mut self, text: &str, font: &FontProps<'_>) {
        todo!()
    }
    fn image(&mut self, src: ImageSource) {
        todo!()
    }
    fn circle(&mut self, point: Point, radius: f64) {
        todo!()
    }
    fn rectangle(&mut self, top_left: Point, bottom_right: Point) {
        todo!()
    }
}
