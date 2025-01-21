use std::{fmt::Debug, vec};

use crate::drawable::BorrowedDrawStyle;

use super::{DrawCommand, Drawing};

mod instruction;
pub use instruction::*;

#[derive(Debug)]
pub struct DrawingIter<'drawing, 'style, Unit> {
    stack: Vec<(
        &'drawing Drawing<'drawing, Unit>,
        BorrowedDrawStyle<'style, Unit>,
    )>,
    used: Vec<&'drawing Drawing<'drawing, Unit>>,
}

impl<'drawing, 'style, Unit> DrawingIter<'drawing, 'style, Unit>
where
    'drawing: 'style,
{
    pub fn new(root: &'drawing Drawing<Unit>) -> Self {
        Self {
            stack: vec![(root, root.style().borrowed())],
            used: Vec::with_capacity(root.num_drawings()),
        }
    }
}

impl<'drawing, 'style, Unit> Iterator for DrawingIter<'drawing, 'style, Unit>
where
    'drawing: 'style,
{
    type Item = DrawingInstruction<'drawing, 'style, Unit>;

    /// This will return drawing command
    fn next(&mut self) -> Option<DrawingInstruction<'drawing, 'style, Unit>> {
        let (node, parent_style) = self.stack.pop()?;

        let node_style = node.style.borrowed().combine_styles(&parent_style);

        for child in node.children.iter().rev() {
            self.stack.push((child, node_style.clone()));
        }

        self.used.push(node);

        let node = self.used.last().unwrap();

        // if the next command is found here, then we yield a drawing instruction
        // otherwise, we call next to find the next drawing instruction.
        match node.command() {
            Some(command) => {
                let instruction = DrawingInstruction::new(command, node_style);
                Some(instruction)
            }
            None => self.next(),
        }
    }
}

#[test]
fn test_drawingiter_style_inheritance() {
    use crate::{
        color::{
            defaults::{BLUE, GREEN, RED},
            Paint,
        },
        drawable::{BorrowedDrawStyle, DrawStyle, PathCommands},
    };
    use pretty_assertions::assert_eq;

    let green: Paint = GREEN.into();
    let red: Paint = RED.into();
    let blue: Paint = BLUE.into();

    // all drawings have a command to show up in the tree.
    let mut root = Drawing::new(
        PathCommands::new(),
        DrawStyle::new(Some(Some(3.0)), Some(Some(green.clone())), Some(None)),
    );

    let mut child_1 = Drawing::new(
        PathCommands::new(),
        DrawStyle::new(None, Some(Some(red.clone())), None),
    );

    let child_1_1 = Drawing::new(
        PathCommands::new(),
        DrawStyle::new(None, None, Some(Some(red.clone()))),
    );

    let child_1_2 = Drawing::new(PathCommands::new(), DrawStyle::new(None, None, None));

    child_1.add_children([child_1_1, child_1_2]);

    let child_2 = Drawing::new(
        PathCommands::new(),
        DrawStyle::new(Some(Some(1.0)), None, Some(Some(blue.clone()))),
    );

    root.add_children([child_1, child_2]);

    let expected_styles = [
        BorrowedDrawStyle::from_raw_parts(Some(Some(&3.0)), Some(Some(&green)), Some(None)), //root
        BorrowedDrawStyle::from_raw_parts(Some(Some(&3.0)), Some(Some(&red)), Some(None)), //child_1
        BorrowedDrawStyle::from_raw_parts(Some(Some(&3.0)), Some(Some(&red)), Some(Some(&red))), //child_1_1
        BorrowedDrawStyle::from_raw_parts(Some(Some(&3.0)), Some(Some(&red)), Some(None)), //child_1_2
        BorrowedDrawStyle::from_raw_parts(Some(Some(&1.0)), Some(Some(&green)), Some(Some(&blue))), //child_2
    ];

    for (instruction, exp_style) in root.instructions().zip(expected_styles) {
        println!("Instruction: {:#?}", instruction);

        assert_eq!(instruction.style(), &exp_style);
    }
}

#[test]
fn test_drawingiter_instruction_yield() {
    use crate::{
        color::{
            defaults::{BLUE, GREEN, RED},
            Paint,
        },
        drawable::{BorrowedDrawStyle, DrawStyle, PathCommands},
    };
    use pretty_assertions::assert_eq;

    let green: Paint = GREEN.into();
    let red: Paint = RED.into();
    let blue: Paint = BLUE.into();

    // root and child_1 do not have a command, so they are not yielded by the iterator.
    // however, their styles are still inherited by their children.
    let mut root = Drawing::from_style(DrawStyle::new(
        Some(Some(3.0)),
        Some(Some(green.clone())),
        Some(None),
    ));

    let mut child_1 = Drawing::from_style(DrawStyle::new(None, Some(Some(red.clone())), None));

    let child_1_1 = Drawing::new(
        PathCommands::new(),
        DrawStyle::new(None, None, Some(Some(red.clone()))),
    );

    let child_1_2 = Drawing::new(PathCommands::new(), DrawStyle::new(None, None, None));

    child_1.add_children([child_1_1, child_1_2]);

    let child_2 = Drawing::new(
        PathCommands::new(),
        DrawStyle::new(Some(Some(1.0)), None, Some(Some(blue.clone()))),
    );

    root.add_children([child_1, child_2]);

    let expected_styles = [
        BorrowedDrawStyle::from_raw_parts(Some(Some(&3.0)), Some(Some(&red)), Some(Some(&red))), //child_1_1
        BorrowedDrawStyle::from_raw_parts(Some(Some(&3.0)), Some(Some(&red)), Some(None)), //child_1_2
        BorrowedDrawStyle::from_raw_parts(Some(Some(&1.0)), Some(Some(&green)), Some(Some(&blue))), //child_2
    ];

    for (instruction, exp_style) in root.instructions().zip(expected_styles) {
        println!("Instruction: {:#?}", instruction);

        assert_eq!(instruction.style(), &exp_style);
    }
}
