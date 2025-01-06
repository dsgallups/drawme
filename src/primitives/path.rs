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
    pub fn move_to(&mut self, point: impl Into<Point>) {
        self.0.push(PathCommand::MoveTo(point.into()))
    }
    pub fn line_to(&mut self, point: impl Into<Point>) {
        self.0.push(PathCommand::LineTo(point.into()))
    }
    pub fn quad_to(&mut self, control: impl Into<Point>, end: impl Into<Point>) {
        self.0.push(PathCommand::QuadTo {
            control: control.into(),
            end: end.into(),
        })
    }
    pub fn curve_to(
        &mut self,
        control_one: impl Into<Point>,
        control_two: impl Into<Point>,
        end: impl Into<Point>,
    ) {
        self.0.push(PathCommand::CurveTo {
            control_one: control_one.into(),
            control_two: control_two.into(),
            end: end.into(),
        })
    }
}

impl<C: Canvas + ?Sized> Drawable<C> for Path {
    fn draw(&self, canvas: &mut C) {
        canvas.path(self)
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
