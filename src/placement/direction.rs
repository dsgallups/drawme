use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

use crate::prelude::DrawUnit;

/// Direction enum for the placement of the object
///
/// Will be used in the future to improve the placement of the heatmap arrowws
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Based on the current location and the current direction, increment to the next location by one.
    pub fn next_coords<T>(&self, current: Vector2<T>) -> Vector2<T>
    where
        T: DrawUnit,
    {
        match self {
            Self::North => Vector2::new(current.x, current.y + T::ONE),
            Self::South => Vector2::new(current.x, current.y - T::ONE),
            Self::West => Vector2::new(current.x - T::ONE, current.y),
            Self::East => Vector2::new(current.x + T::ONE, current.y),
        }
    }
}
