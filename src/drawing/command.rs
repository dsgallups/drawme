use nalgebra::{Scalar, Vector2};

use crate::prelude::*;

#[doc = r#"
A set of possible actions that a [`DrawableBackend`](crate::drawable::DrawableBackend) can execute


There are five "path-like" commands, of which can be considered "atomic" for drawings, which are
- [MoveTo](DrawingCommand::MoveTo)
- [LineTo](DrawingCommand::LineTo)
- [QuadTo](DrawingCommand::QuadTo)
- [CurveTo](DrawingCommand::CurveTo)
- [Close](DrawingCommand::Close)

There are two additional commands, of which are not considered atomic, but are abstract enough to warrant
their own command. These commands are to be executed by themselves, irrespective of an impending list of
path-like commands that have not been issued a [Close](DrawingCommand::Close).
- [CircleAt](DrawingCommand::CircleAt)
- [Text](DrawingCommand::Text)

"#]
#[derive(Debug, PartialEq, Clone)]
pub enum DrawCommand<Unit = f64> {
    Path(Path),
    /// An operation that calls for the drawing of a circle.
    ///
    /// The style property of this command should utilize the same rules as [`Close`](DrawingCommand::Close).
    Circle {
        position: Vector2<Unit>,
        radius: Unit,
    },
    Text {
        text: String,
        start: Vec2<Unit>,
        end: Vec2<Unit>,
        font: RawFontProps,
        rotation: Option<Rotation>,
    },
    Image {
        src: ImageSource,
        props: ImageProps,
    },
}

impl DrawCommand {
    pub fn path(commands: impl Into<Path>) -> Self {
        Self::Path(commands.into())
    }

    pub fn text(
        text: impl Into<String>,
        start: impl Into<Vec2<Unit>>,
        end: impl Into<Vec2<Unit>>,
        props: RawFontProps,
        rotation: Option<Rotation>,
    ) -> Self {
        Self::Text {
            text: text.into(),
            start: start.into(),
            end: end.into(),
            font: props,
            rotation,
        }
    }

    pub fn circle(position: impl IntoVector, radius: f64) -> Self {
        Self::Circle {
            position: position.into_vector(),
            radius,
        }
    }

    pub fn image(image_source: ImageSource, image_props: ImageProps) -> Self {
        Self::Image {
            src: image_source,
            props: image_props,
        }
    }

    pub fn locations(&self) -> Vec<Vector> {
        use DrawCommand::*;
        match self {
            Path(commands) => commands.locations(),
            Circle {
                position: loc,
                radius: _,
            } => vec![loc],
            Text {
                text: _,
                start,
                end,
                font: _,
                rotation: _,
            } => vec![start, end],
            Image { src: _, props } => vec![props.offset()],
        }
    }
    pub fn locations_mut(&mut self) -> Vec<&mut Vector> {
        use DrawCommand::*;
        match self {
            Path(commands) => commands.locations_mut(),
            Circle {
                position: loc,
                radius: _,
            } => vec![loc],
            Text {
                text: _,
                start,
                end,
                font: _,
                rotation: _,
            } => vec![start, end],
            Image { src: _, props } => vec![props.offset_mut()],
        }
    }

    /// If one has drawn the drawing where x is not the right, and y is not down, this will reorient the drawing
    /// to respect the axes.
    pub fn reorient(&mut self, axes: &Axes, total_dimensions: &Dimensions<Unit>) -> &mut Self
    where
        Unit: Neg<Output = Unit>,
    {
        if axes.positive_x() == Direction::East && axes.positive_y() == Direction::South {
            return self;
        }

        if axes.positive_x() != Direction::East && axes.positive_y() != Direction::North {
            unimplemented!("Reorient has not been implemented for these axes.")
        }

        for loc in self.locations_mut() {
            loc.y = total_dimensions.height().clone() - loc.y.clone();
        }

        self
    }
}

impl<Unit: DrawableUnit> DrawableCommand for DrawCommand<Unit> {
    type DrawUnit = Unit;
    fn into_draw_command(self) -> DrawCommand<Self::DrawUnit> {
        self
    }
}

impl<Unit: DrawableUnit> Transformation<Unit> for DrawCommand<Unit> {
    fn rotate(&mut self, rotation: &Rotation) {
        use DrawCommand::*;
        match self {
            Text {
                text: _,
                start: _,
                end: _,
                font: _,
                rotation: rotation_inner,
            } => match rotation_inner {
                Some(ref mut rot) => *rot += *rotation,
                None => *rotation_inner = Some(*rotation),
            },
            Image { src: _, props } => match props.rotation_mut() {
                Some(rot) => *rot += *rotation,
                None => {
                    props.set_rotation(*rotation);
                }
            },
            _ => self
                .locations_mut()
                .into_iter()
                .for_each(|loc| loc.rotate(rotation)),
        }
    }
    fn translate(&mut self, point: &Vec2<Unit>) {
        self.locations_mut()
            .into_iter()
            .for_each(|loc| loc.translate(point))
    }
    fn rotate_around(&mut self, rotation: &Rotation, relative_to: &Vec2<Unit>) {
        self.locations_mut()
            .into_iter()
            .for_each(|loc| loc.rotate_around(rotation, relative_to))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PathCommands<Unit = f64>(Vec<PathCommand<Unit>>);

impl<Unit> PathCommands<Unit> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, PathCommand<Unit>> {
        self.0.iter()
    }
}

impl<Unit> From<Vec<PathCommand<Unit>>> for PathCommands<Unit> {
    fn from(vec: Vec<PathCommand<Unit>>) -> Self {
        Self(vec)
    }
}

impl<Unit, const N: usize> From<[PathCommand<Unit>; N]> for PathCommands<Unit> {
    fn from(value: [PathCommand<Unit>; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl<Unit> IntoIterator for PathCommands<Unit> {
    type Item = PathCommand<Unit>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Unit> FromIterator<PathCommand<Unit>> for PathCommands<Unit> {
    fn from_iter<T: IntoIterator<Item = PathCommand<Unit>>>(iter: T) -> Self {
        PathCommands(iter.into_iter().collect())
    }
}

impl<Unit> From<PathCommands<Unit>> for DrawCommand<Unit> {
    fn from(commands: PathCommands<Unit>) -> Self {
        DrawCommand::path(commands)
    }
}

impl<Unit> DrawableCommand for PathCommands<Unit> {
    type DrawUnit = Unit;
    fn into_draw_command(self) -> DrawCommand<Self::DrawUnit> {
        DrawCommand::path(self)
    }
}

impl<Unit> PathCommands<Unit> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn from_vec(vec: Vec<PathCommand<Unit>>) -> Self {
        Self(vec)
    }

    pub fn locations(&self) -> Vec<&Vec2<Unit>> {
        self.0
            .iter()
            .flat_map(|command| command.locations())
            .collect()
    }

    pub fn locations_mut(&mut self) -> Vec<&mut Vec2<Unit>> {
        self.0
            .iter_mut()
            .flat_map(|command| command.locations_mut())
            .collect()
    }

    pub fn convert<NewUnit: From<Unit>>(self) -> PathCommands<NewUnit> {
        PathCommands(
            self.0
                .into_iter()
                .map(|command| command.convert())
                .collect(),
        )
    }

    pub fn chain(&mut self, other: impl Into<PathCommands<Unit>>) -> &mut Self {
        self.0.extend(other.into().0);
        self
    }

    pub fn add(&mut self, command: impl Into<PathCommand<Unit>>) -> &mut Self {
        self.0.push(command.into());
        self
    }

    pub fn commands(&self) -> &[PathCommand<Unit>] {
        self.0.as_slice()
    }

    pub fn line_to(&mut self, location: impl Into<Vec2<Unit>>) -> &mut Self {
        self.0.push(PathCommand::LineTo(location.into()));
        self
    }
    pub fn move_to(&mut self, location: impl Into<Vec2<Unit>>) -> &mut Self {
        self.0.push(PathCommand::MoveTo(location.into()));
        self
    }

    pub fn quad_to(
        &mut self,
        control: impl Into<Vec2<Unit>>,
        end: impl Into<Vec2<Unit>>,
    ) -> &mut Self {
        self.0.push(PathCommand::QuadTo {
            control: control.into(),
            end: end.into(),
        });
        self
    }

    pub fn curve_to(
        &mut self,
        control_one: impl Into<Vec2<Unit>>,
        control_two: impl Into<Vec2<Unit>>,
        end: impl Into<Vec2<Unit>>,
    ) -> &mut Self {
        self.0.push(PathCommand::CurveTo {
            control_one: control_one.into(),
            control_two: control_two.into(),
            end: end.into(),
        });
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PathCommand<Unit = f64> {
    /// An action to "pick up" the pen, and start at some location.
    MoveTo(Vec2<Unit>),
    /// An action to "draw" from the current location to this location.
    LineTo(Vec2<Unit>),
    /// A quadratic bezier curve
    QuadTo {
        control: Vec2<Unit>,
        end: Vec2<Unit>,
    },
    /// A curve that contains two control points.
    CurveTo {
        control_one: Vec2<Unit>,
        control_two: Vec2<Unit>,
        end: Vec2<Unit>,
    },
}

impl<Unit> PathCommand<Unit> {
    pub fn move_to(loc: impl Into<Vec2<Unit>>) -> Self {
        Self::MoveTo(loc.into())
    }
    pub fn line_to(loc: impl Into<Vec2<Unit>>) -> Self {
        Self::LineTo(loc.into())
    }

    pub fn quad_to(control: impl Into<Vec2<Unit>>, end: impl Into<Vec2<Unit>>) -> Self {
        Self::QuadTo {
            control: control.into(),
            end: end.into(),
        }
    }

    pub fn locations(&self) -> Vec<&Vec2<Unit>> {
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

    pub fn locations_mut(&mut self) -> Vec<&mut Vec2<Unit>> {
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

    pub fn convert<NewUnit: From<Unit>>(self) -> PathCommand<NewUnit> {
        match self {
            PathCommand::MoveTo(loc) => PathCommand::MoveTo(loc.convert()),
            PathCommand::LineTo(loc) => PathCommand::LineTo(loc.convert()),
            PathCommand::QuadTo { control, end } => PathCommand::QuadTo {
                control: control.convert(),
                end: end.convert(),
            },
            PathCommand::CurveTo {
                control_one,
                control_two,
                end,
            } => PathCommand::CurveTo {
                control_one: control_one.convert(),
                control_two: control_two.convert(),
                end: end.convert(),
            },
        }
    }
}
