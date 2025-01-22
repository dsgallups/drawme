use nalgebra::Scalar;

use crate::{color::Paint, style::DrawStyle};

use super::DrawCommand;

/// Used to simplify the drawing process by providing a command and style
#[derive(Debug)]
pub struct DrawingInstruction<'cmd, 'style, Unit: Scalar> {
    command: &'cmd DrawCommand<Unit>,
    style: DrawStyle<'style, Unit>,
}

impl<'cmd, 'style, Unit: Scalar + Copy> DrawingInstruction<'cmd, 'style, Unit> {
    pub fn new(command: &'cmd DrawCommand<Unit>, style: DrawStyle<'style, Unit>) -> Self {
        Self { command, style }
    }

    pub fn command(&self) -> &DrawCommand<Unit> {
        self.command
    }

    pub fn style(&self) -> &DrawStyle<'style, Unit> {
        &self.style
    }

    pub fn stroke_width(&self) -> Option<Unit> {
        self.style().stroke_width()
    }

    pub fn stroke(&self) -> Option<Paint<'_, Unit>> {
        self.style().stroke()
    }

    pub fn fill(&self) -> Option<Paint<'_, Unit>> {
        self.style().fill()
    }
}
