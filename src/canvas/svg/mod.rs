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

use std::{borrow::Cow, fmt::Display, sync::LazyLock};

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

    bounding_box: Vector,
}

impl<N: SvgNode> Svg<N> {
    fn handle_new_element(&mut self, style: DrawStyle<'_>, mut el: N) {
        if let Some(fill) = &style.fill {
            el.push_attribute("fill", fill.css());
        }

        if let Some(stroke) = &style.stroke {
            el.push_attribute("stroke", stroke.css());
        }

        if let Some(sw) = &style.stroke_width {
            el.push_attribute("stroke-width", sw);
        }

        self.root.push_child(el);
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
            root: XmlNode::new("svg").with_attributes([("xmlns", "http://w3.org/2000/svg")]),
            stroke_gradients: vec![],
            fill_gradients: vec![],
            bounding_box: Vector::zeros(),
        }
    }
}

static HEIGHT_R: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"height="(\d+)""#).unwrap());
static WIDTH_R: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"width="(\d+)""#).unwrap());
static VIEWBOX_R: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"viewBox="([^"]+)""#).unwrap());

impl<N: SvgNode> Canvas for Svg<N> {
    fn path(&mut self, style: DrawStyle<'_>, path: &Path) {
        let mut path_el = N::path();

        let path_attr = path
            .iter()
            .map(|command| {
                use PathCommand::*;
                match command {
                    MoveTo(position) => {
                        format!("M {} {}", position.x, position.y)
                    }
                    LineTo(position) => {
                        format!("L {} {}", position.x, position.y)
                    }
                    QuadTo { control, end } => {
                        format!("Q {} {} {} {}", control.x, control.y, end.x, end.y)
                    }
                    CurveTo {
                        control_one,
                        control_two,
                        end,
                    } => {
                        format!(
                            "C {} {} {} {} {} {}",
                            control_one.x,
                            control_one.y,
                            control_two.x,
                            control_two.y,
                            end.x,
                            end.y
                        )
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        path_el.push_attribute("d", path_attr);
        self.handle_new_element(style, path_el);
    }

    fn text(
        &mut self,
        draw_style: DrawStyle<'_>,
        text: &str,
        font: &FontProps<'_>,
        similarity: Isometry,
    ) {
        let style = format!(
            "font-size: {}; font-family: {}; font-weight: {}; font-style: {:?}; font-stretch: {:?}",
            font.size, font.family, font.weight.0, font.style, font.stretch
        );

        let rotation = similarity.rotation.to_rotation_matrix();
        let rotation_str = (rotation != Rotation::identity())
            .then(|| format!("rotate({})", rotation.angle().to_degrees()));

        let translation = similarity.translation;

        let translate_str = format!("translate({}, {})", translation.x, translation.y);
        let mut svg_text = N::text();
        svg_text
            .push_text(Cow::Borrowed(text))
            .push_attribute("style", style);

        if let Some(rotation_str) = rotation_str {
            svg_text
                .push_attribute("transform", format!("{} {}", translate_str, rotation_str))
                .push_attribute("text-anchor", "start");
        } else {
            svg_text.push_attribute("transform", translate_str.as_str());
        }

        self.handle_new_element(draw_style, svg_text);
    }
    fn image(&mut self, _src: &ImageSource) {
        todo!()
    }
    fn circle(&mut self, style: DrawStyle<'_>, point: Point, radius: f64) {
        let mut circle = N::circle();
        circle
            .push_attribute("cx", point.x)
            .push_attribute("cy", point.y)
            .push_attribute("r", radius);

        self.handle_new_element(style, circle);
    }
}
