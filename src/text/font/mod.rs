/*
fontdb features:
       + fontconfig
       + fontconfig-parser
       + fs
       + memmap
       + memmap2
       + std

*/

use core::fmt;
use std::borrow::Cow;

use fontdb::{Style, Weight};

#[derive(Clone, PartialEq, Debug)]
pub struct FontProps<'a> {
    pub size: f64,
    pub weight: Weight,
    pub family: Family<'a>,
    pub style: Style,
    pub stretch: f64,
    pub line_height: f64,
}

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

impl fmt::Display for Family<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let family_str = match self {
            Family::Name(name) => name,
            Family::Cursive => "cursive",
            Family::Fantasy => "fantasy",
            Family::Monospace => "monospace",
            Family::SansSerif => "sans-serif",
            Family::Serif => "serif",
        };
        family_str.fmt(f)
    }
}
