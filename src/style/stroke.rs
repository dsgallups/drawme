use nalgebra::Scalar;

use crate::prelude::*;

pub struct StrokeColor<'a, Unit: Scalar = f64>(Paint<'a, Unit>);

impl<'a, Unit: Scalar> StrokeColor<'a, Unit> {
    pub fn new(paint: Paint<'a, Unit>) -> Self {
        Self(paint)
    }

    pub fn paint(&self) -> &Paint<'_, Unit> {
        &self.0
    }

    pub fn into_paint(self) -> Paint<'a, Unit> {
        self.0
    }
}

impl AsDrawStyle for StrokeColor<'_> {
    fn stroke(&self) -> Option<Paint<'_>> {
        Some(self.0.clone_shallow())
    }
}
