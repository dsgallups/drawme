use std::borrow::Cow;

use crate::prelude::*;

pub struct Fill<'a> {
    paint: Cow<'a, Paint>,
    ovrride: bool,
}

impl<'a> Fill<'a> {
    pub fn new(paint: impl Into<Cow<'a, Paint>>) -> Self {
        Self {
            paint: paint.into(),
            ovrride: false,
        }
    }
}
