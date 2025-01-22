use crate::prelude::*;
use nalgebra::{Point2, Rotation2, Scalar};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "transition"))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Gradient<Unit: Scalar = f64> {
    Linear {
        rot: Rotation2<Unit>,
        colors: Vec<(SolidColor, Unit)>,
    },
    Radial {
        center: Point2<Unit>,
        colors: Vec<(SolidColor, Unit)>,
    },
}

impl Gradient {
    /// Converts the gradient to an [`SvgNode`] type by appending its
    /// properties to the definitions of a [`SvgNode::linear_gradient`].
    #[cfg(feature = "svg")]
    pub fn to_svg_node<N>(&self) -> N
    where
        N: SvgNode,
    {
        match self {
            Gradient::Linear { rot, colors } => {
                let cos = rot.angle().cos();
                let sin = rot.angle().sin();
                let mut lin_el = N::linear_gradient();
                lin_el
                    .push_attribute("x1", format!("{:.2}%", trns_n(-cos)))
                    .push_attribute("y1", format!("{:.2}%", trns_n(sin)))
                    .push_attribute("x2", format!("{:.2}%", trns_n(cos)))
                    .push_attribute("y2", format!("{:.2}%", trns_n(-sin)));

                for (color, offset) in colors {
                    let mut stop = N::stop();
                    stop.push_attribute("offset", format!("{:.2}%", offset * 100.0));

                    // do not use the SolidColor::as_svg_attributes function.
                    match color {
                        SolidColor::Opaque(rgb) => {
                            stop.push_attribute("stop-color", rgb.css());
                        }
                        SolidColor::Alpha(rgba) => {
                            let (r, g, b, a) = rgba.into_tuple();
                            let color = Rgb::new(r, g, b);
                            // stop-opacity is supported by powerpoint. rgba stop colors are not supported.
                            stop.push_attribute("stop-color", color.css())
                                .push_attribute("stop-opacity", a);
                        }
                    }
                    lin_el.push_child(stop);
                }

                lin_el
            }
            Gradient::Radial {
                center: _,
                colors: _,
            } => todo!(),
        }
    }
}

/// Maps [-1,1] to [0, 100]
fn trns_n(f: f64) -> f64 {
    ((f + 1.) / 2.) * 100.
}
