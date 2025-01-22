use crate::prelude::*;

mod circle;
pub use circle::*;

mod path;
use nalgebra::{Scalar, SimdComplexField, SimdRealField};
use num_traits::{ConstOne, NumCast, ToPrimitive};
pub use path::*;

mod rectangle;
pub use rectangle::*;

pub trait Primitive: Sized {
    type Unit: DrawUnit;
    fn with_style<S>(self, style: S) -> Styled<Self, S>
    where
        S: AsDrawStyle<Unit = Self::Unit>,
    {
        Styled { shape: self, style }
    }

    fn draw_with_style<C, S>(&self, style: S, canvas: &mut C)
    where
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle<Unit = Self::Unit>,
    {
        self.draw_primitive(canvas)(style)
    }

    /// Returns a function that will draw onto the canvas with the provided style.
    fn draw_primitive<'c, C, S>(&'c self, canvas: &'c mut C) -> impl FnMut(S) + 'c
    where
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle<Unit = Self::Unit>;
}

pub trait DrawUnit:
    Scalar + SimdComplexField + Copy + ConstOne + ConstTwo + PartialOrd + NumCast
{
    fn rf<N: ToPrimitive>(num: N) -> Self::SimdRealField;

    fn num<N: ToPrimitive>(num: N) -> Self;
}

impl<T> DrawUnit for T
where
    T: Scalar + SimdRealField + Copy + ConstOne + ConstTwo + PartialOrd + NumCast,
    T::SimdRealField: NumCast,
{
    fn rf<N: ToPrimitive>(num: N) -> Self::SimdRealField {
        <Self::SimdRealField as NumCast>::from(num).unwrap()
    }
    fn num<N: ToPrimitive>(num: N) -> Self {
        <Self as NumCast>::from(num).unwrap()
    }
}

pub trait ConstTwo {
    const TWO: Self;
}

impl ConstTwo for f32 {
    const TWO: Self = 2.;
}

impl ConstTwo for f64 {
    const TWO: Self = 2.;
}
