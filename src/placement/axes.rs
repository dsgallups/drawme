use std::ops::Neg;

use super::Direction;

/// Axis enum for the placement of the object
#[derive(Debug, PartialEq)]
pub enum Axis {
    /// The horizontal axis
    X,
    /// The vertical axis
    Y,
}

/// Defines positive x and positive y using the four cardinal directions
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Axes {
    pub(crate) positive_x: Direction,
    pub(crate) positive_y: Direction,
}

impl Default for Axes {
    fn default() -> Self {
        Self {
            positive_x: Direction::East,
            positive_y: Direction::South,
        }
    }
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            East => West,
            West => East,
            North => South,
            South => North,
        }
    }
}

impl Axes {
    pub fn new(positive_x: Direction, positive_y: Direction) -> Self {
        Self {
            positive_x,
            positive_y,
        }
    }

    pub fn positive_x(&self) -> Direction {
        self.positive_x
    }
    pub fn positive_y(&self) -> Direction {
        self.positive_y
    }

    pub fn set_positive_x(&mut self, direction: Direction) -> &mut Self {
        self.positive_x = direction;
        self
    }
    pub fn set_positive_y(&mut self, direction: Direction) -> &mut Self {
        self.positive_y = direction;
        self
    }
}

impl From<(Direction, Direction)> for Axes {
    fn from(value: (Direction, Direction)) -> Self {
        Self::new(value.0, value.1)
    }
}
