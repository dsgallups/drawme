#![doc = r#"
# Text drawing and rendering capabilities.
"#]

use std::{
    borrow::Cow,
    io::Cursor,
    ops::Mul,
    sync::{Arc, LazyLock},
};

use num_traits::{NumCast, ToPrimitive};
use quick_xml::{
    events::{attributes::Attribute, BytesStart, BytesText, Event},
    Writer,
};
use resvg::usvg::fontdb::Database;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

mod position;
pub use position::*;

mod style;
pub use style::*;

pub mod font;

/// todo(dsgallups): use a `fontdb::Database` initilized from outside docgen
/// once cosmic text is removed. The accessing runtime should be driving
/// the database properties.
pub static FONT_DATABASE: LazyLock<Arc<Database>> = LazyLock::new(|| {
    let mut db = Database::new();
    db.load_font_data(include_bytes!("../font/fonts/roboto/Roboto-Regular.ttf").to_vec());

    Arc::new(db)
});

mod textbox;
pub use textbox::*;

/// A piece of text
///
/// The family should be on this text, actually, because the font is an intrinsic property of the text, where the value determines which glyphs of the font to use.
/// Styles on the other hand, like color, and position, etc. are extrinsic properties.
///
/// Text is special/different than something like a shape, because its position is located in its style, not on itself.
///
/// Text is also special because it is an inherently styled property
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Text<'a, Font, Unit = f64> {
    inner: Cow<'a, str>,
    /// Has color, font, and position
    style: TextStyle<'a, Font, Unit>,
}

impl<'a, F, U> Text<'a, F, U> {
    /// Create a new text element for a type that can be turned into a string
    pub fn new(val: impl Into<Cow<'a, str>>, style: impl Into<TextStyle<'a, F, U>>) -> Self {
        Text {
            inner: val.into(),
            style: style.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn style(&self) -> &TextStyle<F, U> {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut TextStyle<F, U> {
        &mut self.style
    }

    pub fn font(&self) -> &F {
        self.style.font()
    }

    pub fn font_mut(&mut self) -> &mut F {
        &mut self.style.font
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.inner
    }
}

#[doc = r#"
This implementation is used to draw text
within some bounding box. This conflicts with another implementation of Text
and should be rectified.
"#]
impl<'font, Font, Unit> Drawable<BoundingBox<Unit>> for Text<Font, Unit>
where
    Pixel: From<Unit> + From<Font::Unit>,
    Unit: DrawableUnit + From<Font::Unit>,
    Font: FontProps<'font>,
    Font::Unit: ToPrimitive + NumCast + Clone + Mul<Percent, Output = Font::Unit>,
{
    type DrawUnit = Unit;
    /// TextStyle has the text position
    fn draw(&self, bounding_box: BoundingBox<Unit>) -> DrawResult<Drawing<Unit>> {
        let text_width: Unit = {
            let mut measurer = FontEngine::borrow().unwrap();
            <font::FontEngine<'_> as TextMetrics<'_, Font, Unit>>::measure_text_width(
                &mut measurer,
                self.as_str(),
                self.font(),
            )
            .map(Into::into)
        }
        .map_err(|fe_error| DrawingError::cant_draw_msg(fe_error.to_string()))?;
        // Defines the bottom-left corner of the text.
        let mut closest = match self.style().position() {
            TextPosition::Align(a) => {
                use RelativeXOrigin::*;

                match a {
                    Left => bounding_box.offset().clone(),
                    Center => {
                        let center = bounding_box.center();

                        let x = center.x - (text_width.clone() / Unit::two());
                        let offset = bounding_box.offset().clone();
                        Vec2::new(x, offset.y)
                    }
                    Right => {
                        let mut pos = bounding_box.offset().clone();
                        pos.x -= text_width.clone();
                        pos
                    }
                }
            }
            TextPosition::Absolute(a) => a.clone().convert(),
        };

        let text_height: Unit = match self.style().line_height() {
            LineHeight::Absolute(amt) => amt.into(),
            LineHeight::Relative(amt) => (self.style().font().size().clone() * amt).into(),
        };

        let two = Unit::two();

        closest.y += (text_height.clone() + bounding_box.height()) / two.clone() - two;

        let mut farthest = closest.clone();
        farthest.x += text_width.clone();
        farthest.y -= text_height;

        let drawing = Drawing::from_command(DrawCommand::text(
            self.text(),
            closest,
            farthest,
            self.font().to_raw_props(),
            None,
        ))
        .with_style(DrawStyle::<Unitless>::fill_only(
            self.style().color().clone(),
        ));

        Ok(drawing)
    }
}

impl XmlComp for Text<PresetFont> {
    fn to_xml_with_attributes(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        additional_attributes: Vec<Attribute<'_>>,
    ) -> Result<(), XmlWriteError> {
        let mut start = BytesStart::new("text");
        start.extend_attributes(additional_attributes);
        start.push_attribute(("style", self.style().to_string().as_str()));

        let c = start.clone();

        let end = c.to_end();
        writer.write_event(Event::Start(start))?;

        writer.write_event(Event::Text(BytesText::new(self.as_str())))?;

        writer.write_event(Event::End(end))?;
        Ok(())
    }
}
