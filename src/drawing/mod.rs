use crate::prelude::*;
use std::ops::Neg;

mod iter;
pub use iter::*;

mod inherited_draw_style;
pub use inherited_draw_style::*;

mod command;
pub use command::*;

#[doc = r#"
Some type of some unit space that can be rendered by an appropriate backend.

Does the following things:
- Defines a set of [`Axes`] that the commands assume
- Defines a fallback, default [`InheritedDrawStyle`] for Close commands
- Defines a list of [`DrawingCommands`]
"#]
#[derive(Debug, Clone)]
pub struct Drawing<'a, Unit = f64> {
    pub(crate) style: InheritedDrawStyle<'a>,
    pub(crate) command: Option<DrawCommand<Unit>>,
    pub(crate) children: Vec<Drawing<'a>>,
}

impl Default for Drawing<'_> {
    /// Creates a new drawing with the following defaults:
    /// relative position is left and top
    fn default() -> Self {
        Self {
            style: InheritedDrawStyle::default(),
            command: None,
            children: Vec::new(),
        }
    }
}

impl<'a, Unit> Drawing<'a, Unit> {
    /// Access to the builder pattern to initialize a drawing.
    pub fn builder() -> DrawingBuilder {
        DrawingBuilder::new()
    }

    pub fn new(command: impl Into<DrawCommand<Unit>>, style: InheritedDrawStyle<'a>) -> Self {
        Self {
            style,
            command: Some(command.into()),
            children: Vec::new(),
        }
    }

    pub fn num_drawings(&self) -> usize {
        self.children
            .iter()
            .fold(1, |acc, child| acc + child.num_drawings())
    }

    pub const fn new_with_children(
        command: Option<DrawCommand>,
        style: InheritedDrawStyle<'a>,
        children: Vec<Drawing>,
    ) -> Self {
        Self {
            style,
            command,
            children,
        }
    }

    /*
    todo
    pub fn from_command<Command>(command: Command) -> Self
    where
        Command: DrawableCommand<DrawUnit = Unit>,
    {
        Self {
            command: Some(command.into_draw_command()),
            ..Default::default()
        }
    }
    */

    pub fn from_style(style: impl Into<InheritedDrawStyle<'a>>) -> Self {
        Self {
            style: style.into(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn add_child(&mut self, command: impl Into<Drawing<'a>>) -> &mut Self {
        self.children.push(command.into())
    }

    pub const fn style(&self) -> InheritedDrawStyle<'_> {
        self.style.clone_shallow()
    }

    pub fn extend_children<Cmds>(&mut self, commands: Cmds) -> &mut Self
    where
        Cmds: IntoIterator,
        Cmds::Item: Into<Drawing<'a>>,
    {
        for command in commands {
            self.add_child(command);
        }
        self
    }

    pub fn command(&self) -> Option<&DrawCommand<Unit>> {
        self.command.as_ref()
    }

    pub fn iter<'s, 'cmd, 'style>(&'s self) -> DrawingIter<'cmd, 'style, Unit>
    where
        's: 'cmd,
        's: 'style,
        'cmd: 'style,
    {
        DrawingIter::new(self)
    }

    pub fn instructions<'s, 'cmd, 'style>(
        &'s self,
    ) -> impl Iterator<Item = DrawingInstruction<'cmd, 'style, Unit>>
    where
        's: 'cmd,
        's: 'style,
        'cmd: 'style,
    {
        self.iter()
    }

    pub fn command_mut(&mut self) -> Option<&mut DrawCommand<Unit>> {
        self.command.as_mut()
    }

    pub fn with_command<Command>(mut self, command: Command) -> Self
    where
        Command: DrawableCommand<DrawUnit = Unit>,
    {
        self.command = Some(command.into_draw_command());
        self
    }

    pub fn set_command<Command>(&mut self, command: Command) -> &mut Self
    where
        Command: DrawableCommand<DrawUnit = Unit>,
    {
        self.command = Some(command.into_draw_command());
        self
    }

    pub fn with_style<Style>(mut self, style: Style) -> Self
    where
        Style: DrawableStyle,
        Unit: From<Style::DrawUnit>,
    {
        self.style = style.into_draw_style();
        self
    }

    pub fn set_style<Style>(&mut self, style: Style) -> &mut Self
    where
        Style: DrawableStyle,
        Unit: From<Style::DrawUnit>,
    {
        self.style = style.into_draw_style();
        self
    }

    pub fn adjust_style(&mut self, f: impl FnOnce(&mut InheritedDrawStyle<Unit>)) -> &mut Self {
        f(&mut self.style);
        self
    }

    pub fn set_fill_color(&mut self, color: impl Into<Paint>) -> &mut Self {
        self.style.set_fill_color(color);
        self
    }

    pub fn set_stroke_color(&mut self, color: impl Into<Paint>) -> &mut Self {
        self.style.set_stroke_color(color);
        self
    }

    pub fn set_stroke_width(&mut self, width: Unit) -> &mut Self {
        self.style.set_stroke_width(width);
        self
    }

    pub fn children(&self) -> &[Drawing<Unit>] {
        self.children.as_slice()
    }

    pub fn convert<NewUnit: From<Unit>>(self) -> Drawing<NewUnit> {
        let style = self.style.convert();
        let command = self.command.map(|c| c.convert());
        let children = self.children.into_iter().map(|c| c.convert()).collect();

        Drawing {
            style,
            command,
            children,
        }
    }
}

impl<Unit: DrawableUnit> Drawing<Unit> {
    /// Returns a box that encapsulates all the commands in O(n).
    ///
    /// The way the bounding box is calculated can be represented by this image:
    ///
    /// ![Bounding Box Calculation][bbox_overview]
    ///
    /// **Safe to unwrap** if the caller KNOWS that the drawing has locational commands.
    ///
    /// Returns `None` if there are no commands, or if the only commands are [`DrawingCommand::Close`].
    #[embed_doc_image("bbox_overview", "readme/bbox_overview.jpg")]
    pub fn bounding_box(&self) -> Option<BoundingBox<Unit>> {
        //self.commands.bounding_box()
        if self.command.is_none() && self.children.is_empty() {
            return None;
        }

        let mut closest: Option<Vec2<&Unit>> = None;
        let mut farthest: Option<Vec2<&Unit>> = None;

        for location in self.locations() {
            match closest {
                Some(ref mut close) => {
                    if &location.x < close.x {
                        close.x = &location.x;
                    }
                    if &location.y < close.y {
                        close.y = &location.y;
                    }
                }
                None => {
                    closest = Some(Vec2 {
                        x: &location.x,
                        y: &location.y,
                    })
                }
            }
            match farthest {
                Some(ref mut far) => {
                    if &location.x > far.x {
                        far.x = &location.x
                    }
                    if &location.y > far.y {
                        far.y = &location.y
                    }
                }
                None => {
                    farthest = Some(Vec2 {
                        x: &location.x,
                        y: &location.y,
                    })
                }
            }
        }

        let (closest, farthest) = (closest?, farthest?);

        let offset = Vec2 {
            x: closest.x.clone(),
            y: closest.y.clone(),
        };

        let dimensions = Dimensions {
            width: farthest.x.clone() - offset.x.clone(),
            height: farthest.y.clone() - offset.y.clone(),
        };

        Some(BoundingBox::new_with_offset(offset, dimensions))
    }

    /// Extremely slow
    pub fn locations(&self) -> Vec<&Vec2<Unit>> {
        match self.command {
            Some(ref c) => {
                let mut loc_vec = c.locations();

                loc_vec.extend(self.children.iter().flat_map(|child| child.locations()));

                loc_vec
            }
            None => self
                .children
                .iter()
                .flat_map(|child| child.locations())
                .collect(),
        }
    }

    /// Extremely slow
    pub fn locations_mut(&mut self) -> Vec<&mut Vec2<Unit>> {
        match self.command {
            Some(ref mut c) => {
                let mut loc_vec = c.locations_mut();

                loc_vec.extend(
                    self.children
                        .iter_mut()
                        .flat_map(|child| child.locations_mut()),
                );

                loc_vec
            }
            None => self
                .children
                .iter_mut()
                .flat_map(|child| child.locations_mut())
                .collect(),
        }
    }

    /// If one has drawn the drawing where x is not the right, and y is not down, this will reorient the drawing to respect the axes.
    pub fn reorient(
        &mut self,
        current_orientation: &Axes,
        total_dimensions: &Dimensions<Unit>,
    ) -> &mut Self
    where
        Unit: Neg<Output = Unit>,
    {
        if let Some(ref mut command) = self.command {
            command.reorient(current_orientation, total_dimensions);
        }
        for child in self.children.iter_mut() {
            child.reorient(current_orientation, total_dimensions);
        }
        self
    }
}

impl<Unit: DrawableUnit> Transformation<Unit> for Drawing<Unit> {
    fn rotate(&mut self, rotation: &Rotation) {
        if let Some(command) = self.command.as_mut() {
            command.rotate(rotation);
        }
        for child in self.children.iter_mut() {
            child.rotate(rotation);
        }
    }
    fn rotate_around(&mut self, rotation: &Rotation, relative_to: &Vec2<Unit>) {
        if let Some(command) = self.command.as_mut() {
            command.rotate_around(rotation, relative_to);
        }
        for child in self.children.iter_mut() {
            child.rotate_around(rotation, relative_to);
        }
    }
    fn translate(&mut self, point: &Vec2<Unit>) {
        if let Some(command) = self.command.as_mut() {
            command.translate(point);
        }
        for child in self.children.iter_mut() {
            child.translate(point);
        }
    }
}

#[test]
fn test_drawing_chain() {
    use pretty_assertions::assert_eq;
    let mut parent = Drawing::builder()
        .stroke_width(0.)
        .fill_color(Paint::rgb(0, 0, 0))
        .build();
    parent.set_command(DrawCommand::path([
        PathCommand::move_to((0., 0.)),
        PathCommand::line_to((5., 5.)),
    ]));

    let mut child = Drawing::builder()
        .stroke_width(0.)
        .fill_color(Paint::rgb(0, 0, 0))
        .build();
    child.set_command(DrawCommand::path([
        PathCommand::move_to((0., 0.)),
        PathCommand::line_to((0., 10.)),
    ]));

    parent.chain(child);

    let expected_commands: [DrawCommand; 2] = [
        DrawCommand::path([
            PathCommand::move_to((0., 0.)),
            PathCommand::line_to((5., 5.)),
        ]),
        DrawCommand::path([
            PathCommand::move_to((0., 0.)),
            PathCommand::line_to((0., 10.)),
        ]),
    ];

    for (act, exp) in parent.instructions().zip(expected_commands.iter()) {
        assert_eq!(act.command(), exp);
    }
}

#[derive(Debug, Clone)]
pub struct DrawingBuilder<Unit = f64> {
    style: Option<InheritedDrawStyle<Unit>>,
}

impl<Unit> Default for DrawingBuilder<Unit> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Unit> DrawingBuilder<Unit> {
    pub fn new() -> Self {
        Self { style: None }
    }

    pub fn set_style(mut self, style: Option<InheritedDrawStyle<Unit>>) -> Self {
        self.style = style;
        self
    }

    pub fn stroke_width(mut self, width: Unit) -> Self {
        match self.style {
            Some(ref mut style) => {
                style.set_stroke_width(width);
            }
            None => {
                let mut style = InheritedDrawStyle::default();
                style.set_stroke_width(width);
                self.style = Some(style);
            }
        }

        self
    }

    pub fn set_stroke_width(mut self, width: Option<Unit>) -> Self {
        match self.style {
            Some(ref mut style) => {
                style.stroke_width = Some(width);
            }
            None => {
                let style = InheritedDrawStyle {
                    stroke_width: Some(width),
                    ..Default::default()
                };
                self.style = Some(style);
            }
        }
        self
    }

    pub fn stroke_color(mut self, color: impl Into<Paint>) -> Self {
        match self.style {
            Some(ref mut style) => {
                style.set_stroke_color(color);
            }
            None => {
                let mut style = InheritedDrawStyle::default();
                style.set_stroke_color(color);
                self.style = Some(style);
            }
        }

        self
    }

    pub fn set_stroke_color(mut self, color: Option<Paint>) -> Self {
        match self.style {
            Some(ref mut style) => {
                style.stroke_color = Some(color);
            }
            None => {
                let style = InheritedDrawStyle {
                    stroke_color: Some(color),
                    ..Default::default()
                };
                self.style = Some(style);
            }
        }
        self
    }

    pub fn fill_color(mut self, color: impl Into<Paint>) -> Self {
        match self.style {
            Some(ref mut style) => {
                style.set_fill_color(color);
            }
            None => {
                let mut style = InheritedDrawStyle::default();
                style.set_fill_color(color);
                self.style = Some(style);
            }
        }
        self
    }

    pub fn set_fill_color(mut self, color: Option<Paint>) -> Self {
        match self.style {
            Some(ref mut style) => {
                style.fill_color = Some(color);
            }
            None => {
                let style = InheritedDrawStyle {
                    fill_color: Some(color),
                    ..Default::default()
                };
                self.style = Some(style);
            }
        }
        self
    }

    pub fn build(self) -> Drawing<Unit> {
        Drawing {
            style: self.style.unwrap_or_default(),
            command: None,
            children: Vec::new(),
        }
    }
}
