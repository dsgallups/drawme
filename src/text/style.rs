use std::{
    fmt::{self},
    ops::Mul,
    str::FromStr,
};

use cosmic_text::{Family, Stretch, Style, Weight};
use num_traits::{NumCast, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

use super::TextPosition;

/// Stylistic properties for text
#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub struct TextStyle<'p, F, Unit = f64> {
    /// The color of the text
    pub color: Paint<'p>,
    pub font: F,
    /// The position of the text
    pub position: TextPosition<Unit>,
}

impl<'a, Unit> Default for TextStyle<FontStyle<'a>, Unit> {
    fn default() -> TextStyle<FontStyle<'a>, Unit> {
        Self {
            color: BLACK.into(),
            font: FontStyle::default(),
            position: TextPosition::default(),
        }
    }
}

impl TextStyle<PresetFont, f64> {
    pub fn strong() -> Self {
        Self {
            font: PresetFont::Strong,
            color: BLACK.into(),
            position: TextPosition::default(),
        }
    }
    pub fn normal() -> Self {
        Self {
            font: PresetFont::Normal,
            color: BLACK.into(),
            position: TextPosition::default(),
        }
    }
}

impl<F, Unit> TextStyle<F, Unit> {
    pub fn new(font: F) -> Self {
        Self {
            font,
            color: BLACK.into(),
            position: TextPosition::default(),
        }
    }

    pub fn new_from_raw(font: F, color: impl Into<Paint>, position: TextPosition<Unit>) -> Self {
        Self {
            font,
            color: color.into(),
            position,
        }
    }

    pub fn align_center(mut self) -> Self {
        self.position = TextPosition::align_center();
        self
    }

    pub fn set_position(&mut self, position: impl Into<TextPosition<Unit>>) -> &mut Self {
        self.position = position.into();
        self
    }

    pub fn set_color(&mut self, color: impl Into<Paint>) -> &mut Self {
        self.color = color.into();
        self
    }

    pub fn color(&self) -> &Paint {
        &self.color
    }

    pub fn font(&self) -> &F {
        &self.font
    }

    pub fn position(&self) -> &TextPosition<Unit> {
        &self.position
    }
}

impl TextStyle<FontStyle<'_>> {
    pub fn set_size(&mut self, size: f64) -> &mut Self {
        self.font.set_size(size);
        self
    }
}

impl<'font, F, Unit> FontProps<'font> for TextStyle<F, Unit>
where
    F: FontProps<'font>,
{
    type Unit = F::Unit;
    fn size(&self) -> &Self::Unit {
        self.font().size()
    }
    fn weight(&self) -> Weight {
        self.font().weight()
    }
    fn family(&self) -> &Family<'font> {
        self.font().family()
    }
    fn stretch(&self) -> Stretch {
        self.font().stretch()
    }
    fn style(&self) -> Style {
        self.font().style()
    }
    fn line_height(&self) -> LineHeight<Self::Unit> {
        self.font().line_height()
    }
}

impl<'a, F> fmt::Display for TextStyle<F>
where
    F: FontProps<'a>,
    F::Unit: fmt::Debug + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"TextStyle {{
    size: {},
    line_height: {:?},
    color: {},
}}"#,
            self.font.size(),
            self.font.line_height(),
            self.color.to_css_prop()
        )
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// A predefined format for a piece of text
/// This is used to define the style and structure of a piece of text
/// without having to specify every single property
///
/// This is useful for things like titles and headings
/// where the style and structure is consistent across the document
pub enum PresetFont {
    Title,
    Subtitle,
    Header1,
    Header2,
    Header3,
    Strong,
    #[default]
    Normal,
    Italic,
}

impl PresetFont {
    pub fn all() -> Vec<PresetFont> {
        vec![
            PresetFont::Title,
            PresetFont::Subtitle,
            PresetFont::Header1,
            PresetFont::Header2,
            PresetFont::Header3,
            PresetFont::Strong,
            PresetFont::Normal,
            PresetFont::Italic,
        ]
    }
}

impl fmt::Display for PresetFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PresetFont::Title => write!(f, "Title"),
            PresetFont::Subtitle => write!(f, "Subtitle"),
            PresetFont::Header1 => write!(f, "Header1"),
            PresetFont::Header2 => write!(f, "Header2"),
            PresetFont::Header3 => write!(f, "Header3"),
            PresetFont::Strong => write!(f, "Strong"),
            PresetFont::Normal => write!(f, "Normal"),
            PresetFont::Italic => write!(f, "Italic"),
        }
    }
}

impl FromStr for PresetFont {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "title" => Ok(PresetFont::Title),
            "subtitle" => Ok(PresetFont::Subtitle),
            "header1" => Ok(PresetFont::Header1),
            "header2" => Ok(PresetFont::Header2),
            "header3" => Ok(PresetFont::Header3),
            "strong" => Ok(PresetFont::Strong),
            "normal" => Ok(PresetFont::Normal),
            "italic" => Ok(PresetFont::Italic),
            _ => Err(()),
        }
    }
}

impl<'font> FontProps<'font> for PresetFont {
    type Unit = f64;
    fn size(&self) -> &f64 {
        use PresetFont::*;
        match self {
            Title => &24.,
            Subtitle => &8.,
            Header1 => &20.,
            Header2 => &16.,
            Header3 => &14.,
            _ => &12.,
        }
    }
    fn weight(&self) -> Weight {
        use PresetFont::*;
        match self {
            Title => Weight::BLACK,
            Strong | Header1 => Weight::BOLD,
            Subtitle => Weight::THIN,
            _ => Weight::NORMAL,
        }
    }
    fn family(&self) -> &Family<'font> {
        &Family::Name(ROBOTO_FAMILY)
    }
    fn stretch(&self) -> Stretch {
        Stretch::Normal
    }
    fn style(&self) -> Style {
        use PresetFont::*;
        match self {
            Italic => Style::Italic,
            _ => Style::Normal,
        }
    }
    fn line_height(&self) -> LineHeight<Self::Unit> {
        LineHeight::relative(1.)
    }
}

/// Defines a font style and position for a string to be rendered.
///
/// This is like [`TextStyle`](super::TextStyle), except a relative point is defined. This is useful
/// since it may not be known what the width of the text is.
pub struct TextStyleRelative<Font, Unit> {
    pub point: Vec2<Unit>,
    pub position: RelativePosition<Unit>,
    pub color: Paint,
    pub font: Font,
}

impl<Font, Unit> TextStyleRelative<Font, Unit> {
    pub fn new(
        point: impl Into<Vec2<Unit>>,
        relative_position: RelativePosition<Unit>,
        color: impl Into<Paint>,
        font: Font,
    ) -> Self {
        Self {
            point: point.into(),
            position: relative_position,
            color: color.into(),
            font,
        }
    }

    pub fn set_relative_position(&mut self, position: RelativePosition<Unit>) -> &mut Self {
        self.position = position;
        self
    }

    pub fn center(point: Vec2<Unit>, color: impl Into<Paint>, font: Font) -> Self {
        Self {
            point,
            position: RelativePosition::center(),
            color: color.into(),
            font,
        }
    }
    pub fn font_style(&self) -> &Font {
        &self.font
    }

    pub fn position(&self) -> &Vec2<Unit> {
        &self.point
    }

    pub fn relative_position(&self) -> &RelativePosition<Unit> {
        &self.position
    }

    pub fn color(&self) -> &Paint {
        &self.color
    }
}

impl<'font, Font, Unit, S> Drawable<&TextStyleRelative<Font, Unit>> for S
where
    Font: FontProps<'font>,
    Font::Unit: ToPrimitive + NumCast + Clone + Mul<Percent, Output = Font::Unit>,
    Pixel: From<Font::Unit>,
    Unit: DrawableUnit + From<Font::Unit>,
    S: AsRef<str>,
{
    type DrawUnit = Unit;
    /// TextStyle has the text position
    fn draw(&self, props: &TextStyleRelative<Font, Unit>) -> DrawResult<Drawing<Unit>> {
        let text_width: Unit = {
            let mut measurer = FontEngine::borrow().unwrap();
            <font::FontEngine<'_> as TextMetrics<'_, Font, Unit>>::measure_text_width(
                &mut measurer,
                self.as_ref(),
                props.font_style(),
            )
            .map(Into::into)
        }
        .map_err(|fe_error| DrawingError::cant_draw_msg(fe_error.to_string()))?;

        let text_height: Unit = match props.font_style().line_height() {
            LineHeight::Absolute(amt) => amt.into(),
            LineHeight::Relative(amt) => (props.font_style().size().clone() * amt).into(),
        };
        // assumes the backend will draw text northwards and eastwards from the closest point.

        // basis x and basis y perform the calculation to account for the text height,
        // so from outside this calculation, it's assumed that (basis_x, basis_y) is located
        // at the correct origin for the relative x origin.

        let mut basis = props.position().clone();

        match props.relative_position().relative_x_origin() {
            RelativeXOrigin::Left => {}
            RelativeXOrigin::Right => basis.x -= text_width.clone(),
            RelativeXOrigin::Center => basis.x -= text_width.clone() / Unit::two(),
        };

        // Text is assumed to be drawn from the bottom left corner
        match props.relative_position().relative_y_origin() {
            RelativeYOrigin::Top => basis.y -= text_height,
            RelativeYOrigin::Bottom => {}
            RelativeYOrigin::Center => basis.y -= text_height / Unit::two(),
        };

        match props.relative_position().offset_x() {
            Some(Offset::Percent(p)) => basis.x += basis.x.clone() * *p,
            Some(Offset::Scalar(u)) => basis.x += u.clone(),
            _ => {}
        }

        match props.relative_position().offset_y() {
            Some(Offset::Percent(p)) => basis.y += basis.y.clone() * *p,
            Some(Offset::Scalar(u)) => basis.y += u.clone(),
            _ => {}
        }

        let mut end = basis.clone();
        end.x += text_width;
        end.y += props.font_style().size().clone().into();

        let drawing = Drawing::from_command(DrawCommand::text(
            self.as_ref(),
            basis,
            end,
            props.font_style().to_raw_props(),
            None,
        ))
        .with_style(Fill::new(props.color()).override_other_props(true));

        Ok(drawing)
    }
}
