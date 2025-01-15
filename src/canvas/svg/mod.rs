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

use std::fmt::Display;

use crate::prelude::*;

mod node;
pub use node::*;

pub type XmlSvg<'a> = Svg<XmlNode<'a>>;

#[derive(Debug)]
pub struct Svg<N> {
    root: N,
    stroke_gradients: Vec<Gradient>,
    fill_gradients: Vec<Gradient>,
}

impl<N: Display> Display for Svg<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for XmlSvg<'_> {
    fn default() -> Self {
        Self {
            root: XmlNode::element("svg").with_attributes([("xmlns", "http://w3.org/2000/svg")]),
            stroke_gradients: vec![],
            fill_gradients: vec![],
        }
    }
}

impl<N: SvgNode> Canvas for Svg<N> {
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
    fn image(&mut self, src: &ImageSource) {
        todo!()
    }
    fn circle(&mut self, point: Point, radius: f64) {
        todo!()
    }
    fn rectangle(&mut self, top_left: Point, bottom_right: Point) {
        todo!()
    }
}
