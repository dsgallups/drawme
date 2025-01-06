/*
fontdb features:
       + fontconfig
       + fontconfig-parser
       + fs
       + memmap
       + memmap2
       + std

*/

use std::borrow::Cow;

use fontdb::Weight;

#[derive(Clone, PartialEq, Debug)]
pub struct FontProps<'a> {
    pub size: f64,
    pub weight: Weight,
    pub family: Family<'a>,
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
