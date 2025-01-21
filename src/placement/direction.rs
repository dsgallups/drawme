use std::ops::{Add, Sub};

use nalgebra::Vector2;
use num_traits::ConstOne;
use serde::{Deserialize, Serialize};

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
    pub const fn next_coords<T>(&self, current: Vector2<T>) -> Vector2<T>
    where
        T: ConstOne + Add<Output = T> + Sub<Output = T>,
    {
        match self {
            Self::North => Vector2::new(current.x, current.y + T::ONE),
            Self::South => Vector2::new(current.x, current.y - T::ONE),
            Self::West => Vector2::new(current.x - T::ONE, current.y),
            Self::East => Vector2::new(current.x + T::ONE, current.y),
        }
    }
}
