use crate::{color::Paint, drawable::BorrowedDrawStyle};

use super::DrawCommand;

/// Used to simplify the drawing process by providing a command and style
#[derive(Debug)]
pub struct DrawingInstruction<'cmd, 'style, Unit> {
    command: &'cmd DrawCommand<Unit>,
    style: BorrowedDrawStyle<'style, Unit>,
}

impl<'cmd, 'style, Unit> DrawingInstruction<'cmd, 'style, Unit> {
    pub fn new(command: &'cmd DrawCommand<Unit>, style: BorrowedDrawStyle<'style, Unit>) -> Self {
        Self { command, style }
    }

    pub fn command(&self) -> &DrawCommand<Unit> {
        self.command
    }

    pub fn style(&self) -> &BorrowedDrawStyle<'style, Unit> {
        &self.style
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
