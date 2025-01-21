use serde::{Deserialize, Serialize};

use crate::placement::{RelativeXOrigin, Vec2};

impl<P: Into<Vec2>> From<P> for TextPosition {
    fn from(pos: P) -> Self {
        TextPosition::Absolute(pos.into())
    }
}

impl From<RelativeXOrigin> for TextPosition {
    fn from(align: RelativeXOrigin) -> Self {
        TextPosition::Align(align)
    }
}

/// Text can be text-aligned (horizontal) or absolutely positioned.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextPosition<Unit = f64> {
    Align(RelativeXOrigin),
    Absolute(Vec2<Unit>),
}

impl<Unit> Default for TextPosition<Unit> {
    fn default() -> Self {
        Self::Align(RelativeXOrigin::Left)
    }
}

impl<Unit> TextPosition<Unit> {
    pub fn align_left() -> Self {
        TextPosition::Align(RelativeXOrigin::Left)
    }

    pub fn align_center() -> Self {
        TextPosition::Align(RelativeXOrigin::Center)
    }

    pub fn align_right() -> Self {
        TextPosition::Align(RelativeXOrigin::Right)
    }

    pub fn pos<Pos: Into<Vec2<Unit>>>(position: Pos) -> Self {
        TextPosition::Absolute(position.into())
    }
}
