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

use std::borrow::Cow;

use crate::prelude::*;

mod node;
pub use node::*;

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
    fn handle_new_element(
        &mut self,
        style: DrawStyle<'_>,
        mut el: N,
        farthest_offset: Option<Vector>,
    ) {
        if let Some(max_offset) = farthest_offset {
            if max_offset > self.bounding_box {
                self.bounding_box = max_offset;
            }
        }

        if let Some(fill) = &style.fill {
            handle_paint(fill, &mut self.fill_gradients, &mut el, "fill");
        }

        if let Some(stroke) = &style.stroke {
            handle_paint(stroke, &mut self.stroke_gradients, &mut el, "stroke");
        }

        if let Some(sw) = &style.stroke_width {
            el.push_attribute("stroke-width", sw);
        }

        self.root.push_child(el);
    }

    pub fn build(mut self) -> Result<N::Output, N::Error> {
        self.root.push_attribute(
            "viewBox",
            format!("0 0 {} {}", self.bounding_box.x, self.bounding_box.y),
        );

        if !self.fill_gradients.is_empty() || !self.stroke_gradients.is_empty() {
            let mut defs = N::defs();
            for (i, gradient) in self.stroke_gradients.iter().enumerate() {
                let mut gn: N = gradient.to_svg_node();
                gn.push_attribute("id", format!("stroke{}", i));
                defs.push_child(gn);
            }

            for (i, gradient) in self.fill_gradients.iter().enumerate() {
                let mut gn: N = gradient.to_svg_node();
                gn.push_attribute("id", format!("fill{}", i));
                defs.push_child(gn);
            }

            self.root.prepend_child(defs);
        }

        self.root.build()
    }
}

fn handle_paint<N: SvgNode>(paint: &Paint, gradients: &mut Vec<Gradient>, el: &mut N, key: &str) {
    match paint {
        Paint::Gradient(gradient) => {
            let gradient_no = gradients
                .iter()
                .position(|g| g == gradient)
                .unwrap_or_else(|| {
                    gradients.push(gradient.clone());
                    gradients.len() - 1
                });

            el.push_attribute(key, format!("url(#{}{})", key, gradient_no));
        }
        Paint::Solid(color) => {
            el.push_attribute(key, color.css());
        }
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

impl<N: SvgNode> Canvas for Svg<N> {
    fn path(&mut self, style: DrawStyle<'_>, path: &Path) {
        let mut path_el = N::path();

        let offset = path.bounding_box();

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
        self.handle_new_element(style, path_el, Some(offset.bottom_right_raw().coords));
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

        self.handle_new_element(draw_style, svg_text, None);
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

        let offset = Vector::new(point.x + radius, point.y + radius);

        self.handle_new_element(style, circle, Some(offset));
    }
}
