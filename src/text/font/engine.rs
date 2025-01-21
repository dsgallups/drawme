use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};

use cosmic_text::{FontSystem, Metrics, Shaping};
use num_traits::{NumCast, ToPrimitive};
use thiserror::Error;
use tracing::info_span;

use super::{FontProps, LineHeight, TextMetrics};

static FONT_ENGINE_INNER: LazyLock<Mutex<FontSystem>> = LazyLock::new(|| {
    let mut fs = FontSystem::new();

    let db = fs.db_mut();
    db.load_font_data(include_bytes!("./fonts/roboto/Roboto-Regular.ttf").to_vec());

    Mutex::new(fs)
});

/// A struct that holds a reference to a static font system.
///
/// This is primarily used to measure the width of text, of which
/// will be converted into a [`DrawingCommand::Text`](crate::drawable::DrawingCommand::Text).
///
/// One will eventually want to lazily load font data (like on the web). This implementation doesn't allow for that.
/// Instead, use a feature flag for the web.
///
/// ### Further notes:
///
/// We should remove cosmic text as a dependency for renderium. the font database needs to be used
///     in many places, and the code to measure text in cosmic text actually doesn't respect
///     the glyph mapping of the font.
pub struct FontEngine<'a> {
    inner: MutexGuard<'a, FontSystem>,
}

impl FontEngine<'_> {
    /// Locks the application-wide mutex. Careful: this may cause deadlock if you are borrowing this more than once!
    pub fn borrow() -> Result<Self, FontEngineError> {
        Ok(Self {
            inner: FONT_ENGINE_INNER.lock()?,
        })
    }
}

impl<'font, Unit, Font> TextMetrics<'font, Font, Unit> for FontEngine<'_>
where
    Font: FontProps<'font>,
    Font::Unit: ToPrimitive + NumCast,
{
    fn measure_text_width(
        &mut self,
        text: &str,
        font: &Font,
    ) -> Result<Font::Unit, FontEngineError> {
        let _span = info_span!("measure_text_width", text = text).entered();

        let size = font.size().to_f32().ok_or(FontEngineError::primitive())?;

        let metrics = match font.line_height() {
            LineHeight::Absolute(height) => {
                Metrics::new(size, height.to_f32().ok_or(FontEngineError::primitive())?)
            }
            LineHeight::Relative(rel_height) => Metrics::relative(size, rel_height.as_f32()),
        };

        let mut buffer = cosmic_text::Buffer::new(&mut self.inner, metrics);
        let mut buffer = buffer.borrow_with(&mut self.inner);

        /*
           For now, we don't use the provided config. Since we're measuring single lines of text, we want the bound to be endless.
           The container may then check if the text is allowed to overflow or not.
           To adjust this behavior, use the method:

           buffer.set_size(bounding_box.width.to_f32(), bounding_box.height.to_f32());
        */

        let attrs = font.as_cosmic_attrs();

        buffer.set_text(text, attrs, Shaping::Basic);

        let mut width: f32 = 0.;
        for run in buffer.layout_runs() {
            if let Some(last_glyph) = run.glyphs.iter().last() {
                width = width.max(last_glyph.x + last_glyph.w);
            }
        }
        <Font::Unit as NumCast>::from(width).ok_or(FontEngineError::primitive())
    }
}

#[derive(Error, Debug)]
pub enum FontEngineError {
    #[error("Something panicked while working with the font engine: {0}")]
    MutexPoisoned(String),
    #[error("There was an error loading a font: {0}")]
    FontNotLoaded(String),
    #[error("The specified font was not found: {0}")]
    FontNotFound(String),
    #[error("The specified unit cannot be cast to or from an f32.")]
    PrimitiveCastF32,
}

impl FontEngineError {
    pub fn primitive() -> Self {
        Self::PrimitiveCastF32
    }
}

impl<'a> From<PoisonError<MutexGuard<'a, FontSystem>>> for FontEngineError {
    fn from(value: PoisonError<MutexGuard<'a, FontSystem>>) -> Self {
        Self::MutexPoisoned(value.to_string())
    }
}
