use nalgebra::Scalar;

use crate::{color::Paint, drawing::InheritedDrawStyle, style::DrawStyle};

use super::DrawCommand;

/// Used to simplify the drawing process by providing a command and style
#[derive(Debug)]
pub struct DrawingInstruction<'cmd, 'style, Unit: Scalar> {
    command: &'cmd DrawCommand<Unit>,
    style: DrawStyle<'style, Unit>,
}

impl<'cmd, 'style, Unit: Scalar> DrawingInstruction<'cmd, 'style, Unit> {
    pub fn new(command: &'cmd DrawCommand<Unit>, style: InheritedDrawStyle<'style, Unit>) -> Self {
        Self { command, style }
    }

    pub fn command(&self) -> &DrawCommand<Unit> {
        self.command
    }

    pub fn style(&self) -> DrawStyle<'style, Unit> {
        self.style.clone_shallow()
    }

    pub fn stroke_width(&self) -> Option<Option<&Unit>> {
        self.style().stroke_width()
    }

    pub fn stroke_color(&self) -> Option<Option<&Paint>> {
        self.style().stroke_color()
    }

    pub fn fill_color(&self) -> Option<Option<&Paint>> {
        self.style().fill_color()
    }
}
