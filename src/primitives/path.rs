use crate::prelude::*;
use nalgebra::{Point2, Scalar};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Path<Unit: Scalar = f64>(Vec<PathCommand<Unit>>);

impl<Unit: Scalar> Default for Path<Unit> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<U: Scalar> Path<U> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, PathCommand<U>> {
        self.0.iter()
    }

    pub fn move_to(&mut self, point: impl IntoPoint<U>) {
        self.0.push(PathCommand::MoveTo(point.into_point()))
    }
    pub fn line_to(&mut self, point: impl IntoPoint<U>) {
        self.0.push(PathCommand::LineTo(point.into_point()))
    }
    pub fn quad_to(&mut self, control: impl IntoPoint<U>, end: impl IntoPoint<U>) {
        self.0.push(PathCommand::QuadTo {
            control: control.into_point(),
            end: end.into_point(),
        })
    }
    pub fn curve_to(
        &mut self,
        control_one: impl IntoPoint<U>,
        control_two: impl IntoPoint<U>,
        end: impl IntoPoint<U>,
    ) {
        self.0.push(PathCommand::CurveTo {
            control_one: control_one.into_point(),
            control_two: control_two.into_point(),
            end: end.into_point(),
        })
    }

    pub fn locations(&self) -> Vec<&Point2<U>> {
        self.0
            .iter()
            .flat_map(|command| command.locations())
            .collect()
    }

    pub fn locations_mut(&mut self) -> Vec<&mut Point2<U>> {
        self.0
            .iter_mut()
            .flat_map(|command| command.locations_mut())
            .collect()
    }
}
impl<U: DrawUnit> Path<U> {
    pub fn bounding_box(&self) -> Rectangle<U> {
        let mut max = Point2::origin();

        let mut commands = self.iter();

        let Some(first) = commands.next() else {
            return Rectangle::zero();
        };
        let mut min = first.get_min();

        for command in commands {
            let min_p = command.get_min();
            let max_p = command.get_max();
            if min_p < min {
                min = min_p;
            }
            if max_p > max {
                max = max_p;
            }
        }

        Rectangle::new(min, max)
    }
}

impl<U: DrawUnit> Primitive for Path<U> {
    type Unit = U;
    fn draw_primitive<'c, C, S>(&'c self, canvas: &'c mut C) -> impl FnMut(S) + 'c
    where
        C: Canvas<Unit = Self::Unit>,
        S: AsDrawStyle<Unit = Self::Unit>,
    {
        |style| canvas.path(style, self)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum PathCommand<Unit: Scalar = f64> {
    MoveTo(Point2<Unit>),
    LineTo(Point2<Unit>),
    QuadTo {
        control: Point2<Unit>,
        end: Point2<Unit>,
    },
    CurveTo {
        control_one: Point2<Unit>,
        control_two: Point2<Unit>,
        end: Point2<Unit>,
    },
}

impl<U: Scalar> PathCommand<U> {
    pub fn move_to(loc: impl IntoPoint<U>) -> Self {
        Self::MoveTo(loc.into_point())
    }
    pub fn line_to(loc: impl IntoPoint<U>) -> Self {
        Self::LineTo(loc.into_point())
    }

    pub fn quad_to(control: impl IntoPoint<U>, end: impl IntoPoint<U>) -> Self {
        Self::QuadTo {
            control: control.into_point(),
            end: end.into_point(),
        }
    }

    pub fn locations(&self) -> Vec<&Point2<U>> {
        use PathCommand::*;
        match self {
            MoveTo(loc) | LineTo(loc) => vec![loc],
            QuadTo { control, end } => vec![control, end],
            CurveTo {
                control_one,
                control_two,
                end,
            } => vec![control_one, control_two, end],
        }
    }

    pub fn locations_mut(&mut self) -> Vec<&mut Point2<U>> {
        use PathCommand::*;
        match self {
            MoveTo(loc) | LineTo(loc) => vec![loc],
            QuadTo { control, end } => vec![control, end],
            CurveTo {
                control_one,
                control_two,
                end,
            } => vec![control_one, control_two, end],
        }
    }
}

impl<U: DrawUnit> PathCommand<U> {
    // Gets the point closest to the origin
    pub fn get_min(&self) -> Point2<U> {
        match self {
            PathCommand::MoveTo(p) | PathCommand::LineTo(p) => *p,
            PathCommand::QuadTo { control, end } => {
                if control < end {
                    *control
                } else {
                    *end
                }
            }
            PathCommand::CurveTo {
                control_one,
                control_two,
                end,
            } => {
                let min_control = if control_one < control_two {
                    control_one
                } else {
                    control_two
                };
                if min_control < end {
                    *min_control
                } else {
                    *end
                }
            }
        }
    }

    /// Returns the point farthest from the origin. Does not account for bends in curves that go beyond points
    pub fn get_max(&self) -> Point2<U> {
        match self {
            PathCommand::MoveTo(p) | PathCommand::LineTo(p) => *p,
            PathCommand::QuadTo { control, end } => {
                if control > end {
                    *control
                } else {
                    *end
                }
            }
            PathCommand::CurveTo {
                control_one,
                control_two,
                end,
            } => {
                let max_control = if control_one > control_two {
                    control_one
                } else {
                    control_two
                };
                if max_control > end {
                    *max_control
                } else {
                    *end
                }
            }
        }
    }
}

impl<Unit: Scalar> From<Vec<PathCommand<Unit>>> for Path<Unit> {
    fn from(vec: Vec<PathCommand<Unit>>) -> Self {
        Self(vec)
    }
}

impl<Unit: DrawUnit, const N: usize> From<[PathCommand<Unit>; N]> for Path<Unit> {
    fn from(value: [PathCommand<Unit>; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl<Unit: DrawUnit> IntoIterator for Path<Unit> {
    type Item = PathCommand<Unit>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Unit: DrawUnit> FromIterator<PathCommand<Unit>> for Path<Unit> {
    fn from_iter<T: IntoIterator<Item = PathCommand<Unit>>>(iter: T) -> Self {
        Path(iter.into_iter().collect())
    }
}
