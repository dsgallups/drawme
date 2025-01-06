use crate::prelude::*;

pub enum DrawCommand {
    Path(()),
    Circle(Circle),
    Image(()),
}
