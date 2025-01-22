use std::ops::Mul;

use num_traits::{NumCast, ToPrimitive};

use crate::{
    color::Paint,
    drawable::{DrawCommand, DrawResult, Drawable, Drawing, DrawingError, Fill},
    font::{self, FontEngine, FontProps, LineHeight, TextMetrics},
    placement::{
        BoundingBox, InBoundingBox, Offset, RelativePosition, RelativeXOrigin, RelativeYOrigin,
        Vec2,
    },
    unit::{DrawableUnit, Percent, Pixel, Two as _},
};

/// Defines a font style and position for a string to be rendered.
pub struct TextBox<Font, Unit> {
    bb: InBoundingBox<Unit>,
    color: Paint,
    font: Font,
}

impl<Font, Unit> TextBox<Font, Unit> {
    pub fn new(bounding_box: InBoundingBox<Unit>, color: impl Into<Paint>, font: Font) -> Self {
        Self {
            bb: bounding_box,
            color: color.into(),
            font,
        }
    }

    pub fn set_bounding_box(&mut self, bounding_box: BoundingBox<Unit>) -> &mut Self {
        self.bb.set_bounding_box(bounding_box);
        self
    }

    pub fn set_position(&mut self, position: RelativePosition<Unit>) -> &mut Self {
        self.bb.set_position(position);
        self
    }

    pub fn from_parts(
        bounding_box: BoundingBox<Unit>,
        position: RelativePosition<Unit>,
        color: impl Into<Paint>,
        font: Font,
    ) -> Self {
        let bb = InBoundingBox::new(bounding_box, position);
        Self {
            bb,
            color: color.into(),
            font,
        }
    }

    pub fn center(bounding_box: BoundingBox<Unit>, color: impl Into<Paint>, font: Font) -> Self {
        let bb = InBoundingBox::new(bounding_box, RelativePosition::center());
        Self {
            bb,
            color: color.into(),
            font,
        }
    }

    pub fn font_style(&self) -> &Font {
        &self.font
    }

    pub fn bounding_box(&self) -> &BoundingBox<Unit> {
        self.bb.bounding_box()
    }

    pub fn position(&self) -> &RelativePosition<Unit> {
        self.bb.position()
    }
    pub fn color(&self) -> &Paint {
        &self.color
    }
}

impl<'font, Font, Unit, S> Drawable<&TextBox<Font, Unit>> for S
where
    Font: FontProps<'font>,
    Font::Unit: ToPrimitive + NumCast + Clone + Mul<Percent, Output = Font::Unit>,
    Pixel: From<Font::Unit>,
    Unit: DrawableUnit + From<Font::Unit>,
    S: AsRef<str>,
{
    type DrawUnit = Unit;
    /// TextStyle has the text position
    fn draw(&self, props: &TextBox<Font, Unit>) -> DrawResult<Drawing<Unit>> {
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
        let basis_x = match props.position().relative_x_origin() {
            RelativeXOrigin::Left => props.bounding_box().western_bound(),
            RelativeXOrigin::Right => props.bounding_box().eastern_bound() - text_width.clone(),
            RelativeXOrigin::Center => {
                let center = props.bounding_box().center();
                center.x - (text_width.clone() / Unit::two())
            }
        };

        let basis_y = match props.position().relative_y_origin() {
            RelativeYOrigin::Top => props.bounding_box().northern_bound() - text_height,
            RelativeYOrigin::Bottom => props.bounding_box().southern_bound(),
            RelativeYOrigin::Center => {
                props.bounding_box().center().y - (text_height / Unit::two())
            }
        };

        let mut start = Vec2::new(basis_x, basis_y);

        match props.position().offset_x() {
            Some(Offset::Percent(p)) => start.x += start.x.clone() * *p,
            Some(Offset::Scalar(u)) => start.x += u.clone(),
            _ => {}
        }

        match props.position().offset_y() {
            Some(Offset::Percent(p)) => start.y += start.y.clone() * *p,
            Some(Offset::Scalar(u)) => start.y += u.clone(),
            _ => {}
        }

        let mut end = start.clone();
        end.x += props.bounding_box().dimensions_ref().width().clone();
        end.y += props.bounding_box().dimensions_ref().height().clone();

        Ok(Drawing::from_command(DrawCommand::text(
            self.as_ref(),
            start,
            end,
            props.font_style().to_raw_props(),
            None,
        ))
        .with_style(Fill::new(props.color()).override_other_props(true)))
    }
}
