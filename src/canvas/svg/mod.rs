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

use std::{fmt::Display, sync::LazyLock};

use crate::prelude::*;

mod node;
pub use node::*;
use regex::Regex;

#[cfg(feature = "xml")]
pub type XmlSvg<'a> = Svg<XmlNode<'a>>;

#[derive(Debug)]
pub struct Svg<N> {
    root: N,
    stroke_gradients: Vec<Gradient>,
    fill_gradients: Vec<Gradient>,

    active_fill: Option<Paint>,
    active_stroke: Option<Paint>,
    active_stroke_width: Option<f64>,
}

impl<N: SvgNode> Svg<N> {
    pub fn handle_new_element(&mut self, el: N) {
        todo!()
    }
}

impl<N: Display> Display for Svg<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(feature = "xml")]
impl Default for XmlSvg<'_> {
    fn default() -> Self {
        Self {
            root: XmlNode::element("svg").with_attributes([("xmlns", "http://w3.org/2000/svg")]),
            stroke_gradients: vec![],
            fill_gradients: vec![],
            active_fill: None,
            active_stroke: None,
            active_stroke_width: None,
        }
    }
}

static HEIGHT_R: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"height="(\d+)""#).unwrap());
static WIDTH_R: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"width="(\d+)""#).unwrap());
static VIEWBOX_R: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"viewBox="([^"]+)""#).unwrap());

impl<N: SvgNode> Canvas for Svg<N> {
    fn set_fill(&mut self, fill: Option<&Paint>) {
        self.active_fill = fill.cloned();
    }
    fn set_stroke_color(&mut self, paint: Option<&Paint>) {
        self.active_stroke = paint.cloned();
    }
    fn set_stroke_width(&mut self, width: Option<f64>) {
        self.active_stroke_width = width;
    }

    fn path(&mut self, path: &Path) {
        let mut path_el = N::path();

        let path_attr = path
            .iter()
            .map(|command| {
                use PathCommand::*;
                match command {
                    MoveTo(position) => {
                        format!("M {} {}", position.x(), position.y())
                    }
                    LineTo(position) => {
                        format!("L {} {}", position.x(), position.y())
                    }
                    QuadTo { control, end } => {
                        format!("Q {} {} {} {}", control.x(), control.y(), end.x(), end.y())
                    }
                    CurveTo {
                        control_one,
                        control_two,
                        end,
                    } => {
                        format!(
                            "C {} {} {} {} {} {}",
                            control_one.x(),
                            control_one.y(),
                            control_two.x(),
                            control_two.y(),
                            end.x(),
                            end.y()
                        )
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        path_el.push_attribute("d", path_attr);
        self.handle_new_element(path_el);
    }
    fn text(&mut self, text: &str, font: &FontProps<'_>) {
        let style = format!(
            "font-size: {}; font-family: {}; font-weight: {}; font-style: {:?}; font-stretch: {:?}",
            font.size, font.family, font.weight.0, font.style, font.stretch
        );
        let rotation_str = rotation.map(|r| format!("rotate({})", r.as_degrees().round_two()));
        let translate_str = format!(
            "translate({}, {})",
            start.x.f64_short(),
            start.y.f64_short()
        );
        let mut svg_text = E::text();
        svg_text.set_inner(text).set_attribute("style", &style);

        if let Some(rotation_str) = rotation_str {
            svg_text
                .set_attribute("transform", format!("{} {}", translate_str, rotation_str))
                .set_attribute("text-anchor", "start");
        } else {
            svg_text.set_attribute("transform", translate_str.as_str());
        }
    }
    fn image(&mut self, src: &ImageSource) {
        todo!()
    }
    fn circle(&mut self, point: Point, radius: f64) {
        let mut circle = N::circle();
        circle
            .push_attribute("cx", point.x())
            .push_attribute("cy", point.y())
            .push_attribute("r", radius);

        self.handle_new_element(circle);
    }
    fn rectangle(&mut self, top_left: Point, bottom_right: Point) {
        todo!()
    }
}
