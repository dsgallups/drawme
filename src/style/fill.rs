use nalgebra::Scalar;

use crate::prelude::*;

pub struct Fill<'a, Unit: Scalar = f64>(Paint<'a, Unit>);

impl<'a, Unit: Scalar> Fill<'a, Unit> {
    pub fn new(paint: impl Into<Paint<'a, Unit>>) -> Self {
        Self(paint.into())
    }

    pub fn paint<'slf>(&'slf self) -> &'slf Paint<'a, Unit> {
        &self.0
    }

    pub fn into_paint(self) -> Paint<'a, Unit> {
        self.0
    }
}

impl<U> AsDrawStyle for Fill<'_, U>
where
    U: Scalar,
{
    type Unit = U;
    fn fill(&self) -> Option<Paint<'_, U>> {
        Some(self.0.clone_shallow())
    }
}
