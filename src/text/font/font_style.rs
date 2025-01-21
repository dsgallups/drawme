use cosmic_text::{fontdb::Query, Attrs, CacheKeyFlags, Family, Stretch, Style, Weight};
use num_traits::One;

use super::{FontProps, LineHeight, ROBOTO_FAMILY};

#[derive(Clone, Debug, PartialEq)]
pub struct FontStyle<'a, Unit = f64> {
    pub family: Family<'a>,
    pub weight: Weight,
    pub stretch: Stretch,
    pub style: Style,
    /// The height of the font in pixels
    pub size: Unit,
    /// The line height of the font as a multiple of the font size, or as a scalar of the font size.
    pub line_height: LineHeight<Unit>,
}

impl Default for FontStyle<'static> {
    fn default() -> Self {
        Self {
            family: Family::Name(ROBOTO_FAMILY),
            weight: Weight::NORMAL,
            stretch: Stretch::Normal,
            style: Style::Normal,
            size: 15.,
            line_height: LineHeight::relative(1.),
        }
    }
}

impl FontStyle<'_> {
    pub fn default_with_size(font_size: f64) -> Self {
        FontStyle {
            size: font_size,
            ..Default::default()
        }
    }
}

impl<'a, Unit> FontStyle<'a, Unit> {
    /// the name should be a TTF `Family Name` (TTF ID 1).
    ///
    /// Takes in a family string and a font_size
    pub fn new<'b: 'a>(family: &'b str, font_size: Unit) -> Self
    where
        Unit: One,
    {
        Self {
            family: Family::Name(family),
            weight: Weight::NORMAL,
            stretch: Stretch::Normal,
            style: Style::Normal,
            size: font_size,
            line_height: LineHeight::relative(1.),
        }
    }

    pub fn set_size(&mut self, size: Unit) -> &mut Self {
        self.size = size;
        self
    }

    pub fn set_weight(&mut self, weight: Weight) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn as_fontdb_query<'b>(&'a self) -> Query<'b>
    where
        'a: 'b,
    {
        let ref_slice = std::slice::from_ref(&self.family);
        Query {
            families: ref_slice,
            weight: self.weight,
            stretch: self.stretch,
            style: self.style,
        }
    }

    pub fn as_cosmic_attrs<'b>(&'a self) -> Attrs<'b>
    where
        'a: 'b,
    {
        Attrs {
            color_opt: None,
            family: self.family,
            stretch: self.stretch,
            style: self.style,
            weight: self.weight,
            metadata: 0,
            cache_key_flags: CacheKeyFlags::empty(),
            metrics_opt: None,
        }
    }

    pub fn with_size<'b, NewUnit>(self, font_size: NewUnit) -> FontStyle<'b, NewUnit>
    where
        'a: 'b,
        Unit: Into<NewUnit>,
    {
        FontStyle {
            family: self.family,
            weight: self.weight,
            stretch: self.stretch,
            style: self.style,
            size: font_size,
            line_height: self.line_height.convert(),
        }
    }
}

impl<'font, FSUnit: Clone> FontProps<'font> for FontStyle<'font, FSUnit> {
    type Unit = FSUnit;
    fn size(&self) -> &FSUnit {
        &self.size
    }
    fn weight(&self) -> Weight {
        self.weight
    }
    fn family(&self) -> &Family<'font> {
        &self.family
    }
    fn stretch(&self) -> Stretch {
        self.stretch
    }
    fn style(&self) -> Style {
        self.style
    }

    fn line_height(&self) -> LineHeight<FSUnit> {
        self.line_height.clone()
    }
}

// the reference to the font style must be at least as useful as the reference to the font
impl<'font, FSUnit: Clone> FontProps<'font> for &FontStyle<'font, FSUnit> {
    type Unit = FSUnit;
    fn size(&self) -> &FSUnit {
        &self.size
    }
    fn weight(&self) -> Weight {
        self.weight
    }
    fn family(&self) -> &Family<'font> {
        &self.family
    }
    fn stretch(&self) -> Stretch {
        self.stretch
    }
    fn style(&self) -> Style {
        self.style
    }

    fn line_height(&self) -> LineHeight<FSUnit> {
        self.line_height.clone()
    }
}
