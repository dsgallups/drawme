#![doc = r#"Contains methods to load, measure, and rasterize fonts"#]
mod engine;

pub use cosmic_text::{fontdb::Query, Attrs, CacheKeyFlags, Family, Stretch, Style, Weight};
pub use engine::*;

pub mod font_source;
pub use font_source::*;

pub mod line_height;
pub use line_height::*;

pub mod font_style;
pub use font_style::*;

use crate::unit::Pixel;

pub trait FontProps<'font> {
    type Unit;
    fn size(&self) -> &Self::Unit;
    fn weight(&self) -> Weight;
    fn family(&self) -> &Family<'font>;
    fn stretch(&self) -> Stretch;
    fn style(&self) -> Style;
    fn line_height(&self) -> LineHeight<Self::Unit>;
    fn as_fontdb_query<'s, 'query>(&'s self) -> Query<'query>
    where
        's: 'query,
        'font: 'query,
    {
        let family = self.family();
        let ref_slice = std::slice::from_ref(family);
        Query {
            families: ref_slice,
            weight: self.weight(),
            stretch: self.stretch(),
            style: self.style(),
        }
    }

    fn as_cosmic_attrs<'attrs>(&self) -> Attrs<'attrs>
    where
        'font: 'attrs,
    {
        Attrs {
            color_opt: None,
            family: *self.family(),
            stretch: self.stretch(),
            style: self.style(),
            weight: self.weight(),
            metadata: 0,
            cache_key_flags: CacheKeyFlags::empty(),
            metrics_opt: None,
        }
    }

    /// This function raises a question about when font should be measured.
    /// Should it be measured when generating the DrawingCommands,
    /// or should it be measured when rendering the DrawingCommands?
    ///
    /// Currently, the text position is measured during the generation of the DrawingCommands.
    ///
    ///
    /// Per conversion in PR #286, we discussed this and determined that the generation of drawingcommands
    /// for text should be primarily during the "to drawing commands" phase of an object, not during the "display" phase,
    /// where the backend receives the drawable commands.
    fn to_raw_props(&self) -> RawFontProps
    where
        Self::Unit: Clone,
        Pixel: From<Self::Unit>,
    {
        let family_str = match self.family() {
            Family::Name(name) => name,
            Family::Cursive => "cursive",
            Family::Fantasy => "fantasy",
            Family::Monospace => "monospace",
            Family::SansSerif => "sans-serif",
            Family::Serif => "serif",
        };
        RawFontProps {
            size: <Pixel as From<Self::Unit>>::from(self.size().clone()),
            weight: self.weight(),
            family_str: family_str.to_string(),
            stretch: self.stretch(),
            style: self.style(),
            line_height: self.line_height().convert(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawFontProps {
    pub size: Pixel,
    pub weight: Weight,
    pub family_str: String,
    pub stretch: Stretch,
    pub style: Style,
    pub line_height: LineHeight<Pixel>,
}

/// Not considering rotation
pub trait TextMetrics<'font, Font, Unit>
where
    Font: FontProps<'font>,
{
    fn measure_text_width(
        &mut self,
        text: &str,
        font: &Font,
    ) -> Result<Font::Unit, FontEngineError>;
}
