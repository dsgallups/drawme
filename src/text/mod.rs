use std::borrow::Cow;

use fontdb::{Family, Query, Stretch, Style, Weight};
use nalgebra::{Rotation2, Scalar};

use crate::{
    prelude::{BoundingBox, CommonConsts, RelativeXOrigin, BLACK},
    style::{DrawStyle, Fill},
};

pub mod font;

pub struct TextLayout<'text, 'style, 'family, Unit: Scalar = f64> {
    text: Cow<'text, str>,
    style: TextStyle<'style, 'family, Unit>,
}

impl<Unit: Scalar + CommonConsts> Default for TextStyle<'_, '_, Unit> {
    fn default() -> Self {
        Self {
            family: Family::Serif,
            size: Unit::ONE,
            stretch: Stretch::Normal,
            style: Style::Normal,
            weight: Weight::NORMAL,
            line_height: Unit::ONE,
            bounding_box: BoundingBox::zero(),
            alignment: RelativeXOrigin::Left,
            rotation: Rotation2::identity(),
            draw_style: DrawStyle::from_fill(Fill::new(BLACK)),
        }
    }
}

pub struct TextStyle<'style, 'family, Unit: Scalar = f64> {
    pub family: Family<'family>,
    pub size: Unit,
    pub weight: Weight,
    pub stretch: Stretch,
    pub style: Style,
    pub line_height: Unit,
    pub bounding_box: BoundingBox<Unit>,
    pub alignment: RelativeXOrigin,
    //should the bounding box be rotated as well?
    pub rotation: Rotation2<Unit>,
    pub draw_style: DrawStyle<'style, Unit>,
}

impl<'style, 'family, U: Scalar + Copy> TextStyle<'style, 'family, U> {
    pub fn draw_style(&self) -> &DrawStyle<'style, U> {
        &self.draw_style
    }
    pub fn family(&self) -> &Family<'family> {
        &self.family
    }
    pub fn size(&self) -> U {
        self.size
    }
    pub fn weight(&self) -> Weight {
        self.weight
    }
    pub fn style(&self) -> Style {
        self.style
    }
    pub fn line_height(&self) -> U {
        self.line_height
    }
    pub fn bounding_box(&self) -> BoundingBox<U> {
        self.bounding_box
    }
    pub fn alignment(&self) -> RelativeXOrigin {
        self.alignment
    }
    pub fn rotation(&self) -> Rotation2<U> {
        self.rotation
    }
}

impl<'style, 'family, U: Scalar + Copy> TextStyle<'style, 'family, U> {
    pub fn to_query(&self) -> Query {
        todo!()
    }
}

/*
#[derive(Clone, PartialEq, Debug)]
pub enum Family<'a> {
    Name(Cow<'a, str>),
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}
impl<'a> Family<'a> {
    pub fn name(name: impl Into<Cow<'a, str>>) -> Self {
        Self::Name(name.into())
    }
}
*/
