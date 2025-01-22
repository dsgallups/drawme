use nalgebra::{Point2, Rotation2, Scalar};

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
pub enum DrawCommand<Unit: Scalar = f64> {
    Path(Path<Unit>),
    /// An operation that calls for the drawing of a circle.
    ///
    /// The style property of this command should utilize the same rules as [`Close`](DrawingCommand::Close).
    Circle {
        position: Point2<Unit>,
        radius: Unit,
    },
    // Text {
    //         text: String,
    //         start: Point2<Unit>,
    //         end: Point2<Unit>,
    //         font: RawFontProps,
    //         rotation: Option<Rotation2<Unit>>,
    //     },
    Text {
        text: String,
        start: Point2<Unit>,
        end: Point2<Unit>,
        font: String,
        rotation: Option<Rotation2<Unit>>,
    },
    Image {
        src: ImageSource,
        props: ImageProps,
    },
}

impl<U: Scalar> DrawCommand<U> {
    pub fn path(commands: impl Into<Path<U>>) -> Self {
        Self::Path(commands.into())
    }

    // pub fn text(
    //     text: impl Into<String>,
    //     start: Option<Vector2<f64>>,
    //     end: Option<Vector2<f64>>,
    //     props: RawFontProps,
    //     rotation: Option<Rotation2<f64>>,
    // ) -> Self {
    //     Self::Text {
    //         text: text.into(),
    //         start: start.into(),
    //         end: end.into(),
    //         font: props,
    //         rotation,
    //     }
    // }

    pub fn circle(position: impl IntoPoint<U>, radius: U) -> Self {
        Self::Circle {
            position: position.into_point(),
            radius,
        }
    }

    pub fn image(image_source: ImageSource, image_props: ImageProps) -> Self {
        Self::Image {
            src: image_source,
            props: image_props,
        }
    }

    pub fn locations(&self) -> Vec<&Point2<U>> {
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
            // Image { src: _, props } => vec![props.offset()],
            Image { .. } => todo!(),
        }
    }
    pub fn locations_mut(&mut self) -> Vec<&mut Point2<U>> {
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
            // Image { src: _, props } => vec![props.offset_mut()],
            Image { .. } => todo!(),
        }
    }

    /*
    /// If one has drawn the drawing where x is not the right, and y is not down, this will reorient the drawing
    /// to respect the axes.
    pub fn reorient(&mut self, axes: &Axes, total_dimensions: &Dimensions<Unit>) -> &mut Self {
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
    }*/
}

// impl<Unit: DrawableUnit> DrawableCommand for DrawCommand<Unit> {
//     type DrawUnit = Unit;
//     fn into_draw_command(self) -> DrawCommand<Self::DrawUnit> {
//         self
//     }
// }

// impl<Unit: DrawableUnit> Transformation<Unit> for DrawCommand<Unit> {
//     fn rotate(&mut self, rotation: &Rotation) {
//         use DrawCommand::*;
//         match self {
//             Text {
//                 text: _,
//                 start: _,
//                 end: _,
//                 font: _,
//                 rotation: rotation_inner,
//             } => match rotation_inner {
//                 Some(ref mut rot) => *rot += *rotation,
//                 None => *rotation_inner = Some(*rotation),
//             },
//             Image { src: _, props } => match props.rotation_mut() {
//                 Some(rot) => *rot += *rotation,
//                 None => {
//                     props.set_rotation(*rotation);
//                 }
//             },
//             _ => self
//                 .locations_mut()
//                 .into_iter()
//                 .for_each(|loc| loc.rotate(rotation)),
//         }
//     }
//     fn translate(&mut self, point: &Vec2<Unit>) {
//         self.locations_mut()
//             .into_iter()
//             .for_each(|loc| loc.translate(point))
//     }
//     fn rotate_around(&mut self, rotation: &Rotation, relative_to: &Vec2<Unit>) {
//         self.locations_mut()
//             .into_iter()
//             .for_each(|loc| loc.rotate_around(rotation, relative_to))
//     }
// }
