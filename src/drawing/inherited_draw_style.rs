#[cfg(feature = "serde")]
use serde::Serialize;

use crate::prelude::*;

/// Defines how a drawing should inherit and display itself visually
///
/// Internally, it's a double wrapped option
///
/// Some(Some(T)) implies that the value is set
///
/// Some(None) implies that the value is set to nothing
///
/// None implies that the value is not set and may be overridden by a parent
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InheritedDrawStyle<'a, Unit = f64> {
    pub stroke_width: Option<Option<Unit>>,
    pub stroke_color: Option<Option<Paint<'a>>>,
    pub fill_color: Option<Option<Paint<'a>>>,
}

impl InheritedDrawStyle<'_> {
    pub const fn clone_shallow(&self) -> InheritedDrawStyle<'_> {
        InheritedDrawStyle {
            stroke_width: self.stroke_width,
            stroke_color: match self.stroke_color {
                None => None,
                Some(None) => Some(None),
                Some(Some(s)) => Some(Some(s.clone_shallow())),
            },
            fill_color: match self.fill_color {
                None => None,
                Some(None) => Some(None),
                Some(Some(s)) => Some(Some(s.clone_shallow())),
            },
        }
    }
}

impl Default for InheritedDrawStyle<'_> {
    fn default() -> Self {
        Self {
            stroke_width: None,
            stroke_color: None,
            fill_color: None,
        }
    }
}
