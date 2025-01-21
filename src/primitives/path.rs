use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Path(Vec<PathCommand>);

impl Path {
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

    pub fn iter(&self) -> std::slice::Iter<'_, PathCommand> {
        self.0.iter()
    }

    pub fn move_to(&mut self, point: impl IntoPoint) {
        self.0.push(PathCommand::MoveTo(point.into_point()))
    }
    pub fn line_to(&mut self, point: impl IntoPoint) {
        self.0.push(PathCommand::LineTo(point.into_point()))
    }
    pub fn quad_to(&mut self, control: impl IntoPoint, end: impl IntoPoint) {
        self.0.push(PathCommand::QuadTo {
            control: control.into_point(),
            end: end.into_point(),
        })
    }
    pub fn curve_to(
        &mut self,
        control_one: impl IntoPoint,
        control_two: impl IntoPoint,
        end: impl IntoPoint,
    ) {
        self.0.push(PathCommand::CurveTo {
            control_one: control_one.into_point(),
            control_two: control_two.into_point(),
            end: end.into_point(),
        })
    }

    pub fn bounding_box(&self) -> Rectangle {
        let mut max = Point::origin();

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

impl Primitive for Path {
    fn draw_primitive<'c, C, S>(&'c self, canvas: &'c mut C) -> impl FnMut(S) + 'c
    where
        C: Canvas,
        S: AsDrawStyle,
    {
        |style| canvas.path(style, self)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadTo {
        control: Point,
        end: Point,
    },
    CurveTo {
        control_one: Point,
        control_two: Point,
        end: Point,
    },
}

impl PathCommand {
    // Gets the point closest to the origin
    pub fn get_min(&self) -> Point {
        use PathCommand as P;
        match self {
            P::MoveTo(p) | P::LineTo(p) => *p,
            P::QuadTo { control, end } => {
                if control < end {
                    *control
                } else {
                    *end
                }
            }
            P::CurveTo {
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
    pub fn get_max(&self) -> Point {
        use PathCommand as P;
        match self {
            P::MoveTo(p) | P::LineTo(p) => *p,
            P::QuadTo { control, end } => {
                if control > end {
                    *control
                } else {
                    *end
                }
            }
            P::CurveTo {
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
