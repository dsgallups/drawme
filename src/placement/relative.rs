use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::prelude::DrawUnit;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativePosition<Unit = f64> {
    relative_x: RelativeXOrigin,
    offset_x: Option<Offset<Unit>>,
    relative_y: RelativeYOrigin,
    offset_y: Option<Offset<Unit>>,
}

impl<Unit> Default for RelativePosition<Unit> {
    fn default() -> Self {
        Self {
            relative_x: RelativeXOrigin::Left,
            offset_x: None,
            relative_y: RelativeYOrigin::Top,
            offset_y: None,
        }
    }
}

impl<Unit: DrawUnit> RelativePosition<Unit> {
    pub fn new(
        relative_x: RelativeXOrigin,
        offset_x: Offset<Unit>,
        relative_y: RelativeYOrigin,
        offset_y: Offset<Unit>,
    ) -> Self {
        Self {
            relative_x,
            offset_x: Some(offset_x),
            relative_y,
            offset_y: Some(offset_y),
        }
    }
    pub fn relative_x_origin(&self) -> RelativeXOrigin {
        self.relative_x
    }
    pub fn offset_x(&self) -> Option<Offset<Unit>> {
        self.offset_x
    }
    pub fn relative_y_origin(&self) -> RelativeYOrigin {
        self.relative_y
    }
    pub fn offset_y(&self) -> Option<Offset<Unit>> {
        self.offset_y
    }

    pub fn center() -> Self {
        Self {
            relative_x: RelativeXOrigin::Center,
            offset_x: None,
            relative_y: RelativeYOrigin::Center,
            offset_y: None,
        }
    }

    pub fn from_origins(relative_x: RelativeXOrigin, relative_y: RelativeYOrigin) -> Self {
        Self {
            relative_x,
            offset_x: None,
            relative_y,
            offset_y: None,
        }
    }
    pub fn with_offset_x(mut self, offset_x: Offset<Unit>) -> Self {
        self.offset_x = Some(offset_x);
        self
    }
    pub fn with_offset_y(mut self, offset_y: Offset<Unit>) -> Self {
        self.offset_y = Some(offset_y);
        self
    }
}

impl<Unit: DrawUnit + Float + FromStr> FromStr for RelativePosition<Unit> {
    type Err = ();
    /// Valid examples:
    /// - top 75px left 100%
    /// - top left 100%
    /// - top 50% left
    /// - top left
    /// ```rust
    /// use std::str::FromStr;
    /// use renderium::prelude::*;
    ///
    /// let pos = RelativePosition::from_str("top 75px left 100%").unwrap();
    /// assert_eq!(pos.relative_x_origin(), RelativeXOrigin::Left);
    /// assert_eq!(pos.offset_x(), Some(&Offset::percent(1.)));
    /// assert_eq!(pos.relative_y_origin(), RelativeYOrigin::Top);
    /// assert_eq!(pos.offset_y(), Some(&Offset::pixel(75.)));
    ///
    /// let pos = RelativePosition::from_str("center left 100%").unwrap();
    /// assert_eq!(pos.relative_x_origin(), RelativeXOrigin::Left);
    /// assert_eq!(pos.offset_x(), Some(&Offset::percent(1.0)));
    /// assert_eq!(pos.relative_y_origin(), RelativeYOrigin::Center);
    /// assert_eq!(pos.offset_y(), None);
    ///
    /// let pos = RelativePosition::from_str("top 50% right").unwrap();
    /// assert_eq!(pos.relative_x_origin(), RelativeXOrigin::Right);
    /// assert_eq!(pos.offset_x(), None);
    /// assert_eq!(pos.relative_y_origin(), RelativeYOrigin::Top);
    /// assert_eq!(pos.offset_y(), Some(&Offset::percent(0.5)));
    ///
    /// let pos = RelativePosition::from_str("top left").unwrap();
    /// assert_eq!(pos.relative_x_origin(), RelativeXOrigin::Left);
    /// assert_eq!(pos.offset_x(), None);
    /// assert_eq!(pos.relative_y_origin(), RelativeYOrigin::Top);
    /// assert_eq!(pos.offset_y(), None);
    ///
    /// assert!(RelativePosition::from_str("center 20px left").is_err());
    ///
    /// let pos = RelativePosition::from_str("center").unwrap();
    /// assert_eq!(pos.relative_x_origin(), RelativeXOrigin::Center);
    /// assert_eq!(pos.offset_x(), None);
    /// assert_eq!(pos.relative_y_origin(), RelativeYOrigin::Center);
    /// assert_eq!(pos.offset_y(), None);
    ///
    /// let pos = RelativePosition::from_str("right").unwrap();
    /// assert_eq!(pos.relative_x_origin(), RelativeXOrigin::Right);
    /// assert_eq!(pos.offset_x(), None);
    /// assert_eq!(pos.relative_y_origin(), RelativeYOrigin::Top);
    /// assert_eq!(pos.offset_y(), None);
    ///
    /// let pos = RelativePosition::from_str("bottom").unwrap();
    /// assert_eq!(pos.relative_x_origin(), RelativeXOrigin::Left);
    /// assert_eq!(pos.offset_x(), None);
    /// assert_eq!(pos.relative_y_origin(), RelativeYOrigin::Bottom);
    /// assert_eq!(pos.offset_y(), None);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let first = parts.next().ok_or(())?;
        let Ok(relative_y) = RelativeYOrigin::from_str(first) else {
            // will never be center
            let relative_x = RelativeXOrigin::from_str(first).map_err(|_| ())?;

            let Some(second) = parts.next() else {
                return Ok(Self {
                    relative_x,
                    ..Default::default()
                });
            };

            let offset_x = Offset::from_str(second).map_err(|_| ())?;

            return Ok(Self {
                relative_x,
                offset_x: Some(offset_x),
                ..Default::default()
            });
        };

        let Some(second) = parts.next() else {
            if relative_y == RelativeYOrigin::Center {
                return Ok(Self::center());
            } else {
                return Ok(Self {
                    relative_y,
                    ..Default::default()
                });
            }
        };

        let Ok(offset_y) = Offset::from_str(second) else {
            let relative_x = RelativeXOrigin::from_str(second).map_err(|_| ())?;

            let Some(third) = parts.next() else {
                return Ok(Self {
                    relative_x,
                    relative_y,
                    ..Default::default()
                });
            };

            let offset_x = Offset::from_str(third).map_err(|_| ())?;

            return Ok(Self {
                relative_x,
                offset_x: Some(offset_x),
                relative_y,
                ..Default::default()
            });
        };

        // a center position cannot hae an offset
        if relative_y == RelativeYOrigin::Center {
            return Err(());
        };

        let Some(third) = parts.next() else {
            return Ok(Self {
                relative_y,
                offset_y: Some(offset_y),
                ..Default::default()
            });
        };

        let relative_x = RelativeXOrigin::from_str(third)?;

        let Some(fourth) = parts.next() else {
            return Ok(Self {
                relative_x,
                relative_y,
                offset_y: Some(offset_y),
                ..Default::default()
            });
        };

        let offset_x = Offset::from_str(fourth)?;

        if relative_x == RelativeXOrigin::Center {
            return Err(());
        }

        Ok(Self {
            relative_x,
            offset_x: Some(offset_x),
            relative_y,
            offset_y: Some(offset_y),
        })
    }
}

impl fmt::Display for RelativePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // x then y
        match self.offset_x() {
            Some(offset) => write!(f, "{} {}", self.relative_x_origin(), offset),
            None => self.relative_x_origin().fmt(f),
        }?;
        write!(f, " ")?;
        match self.offset_y() {
            Some(offset) => write!(f, "{} {}", self.relative_y_origin(), offset),
            None => self.relative_y_origin().fmt(f),
        }?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Offset<Unit = f64> {
    Percent(Unit),
    Scalar(Unit),
}

impl<Unit: Float> Offset<Unit> {
    /// Takes a percent (1.1 is 110%)
    pub const fn percent(amt: Unit) -> Self {
        Self::Percent(amt)
    }

    pub const fn scalar(amt: Unit) -> Self {
        Self::Scalar(amt)
    }
}

impl<Unit: DrawUnit + FromStr + Float> FromStr for Offset<Unit> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.strip_suffix('%').is_some() {
            return Ok(Offset::Percent(
                Unit::from_str(s).map_err(|_| ())? / Unit::ONE_HUNDO,
            ));
        }
        let pixel = s.parse().map_err(|_| ())?;
        Ok(Offset::Scalar(pixel))
    }
}

impl<Unit: DrawUnit + fmt::Display + Float> fmt::Display for Offset<Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Offset::Percent(p) => {
                write!(f, "{}%", *p * Unit::ONE_HUNDO)
            }
            Offset::Scalar(p) => {
                write!(f, "{}", p)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum RelativeXOrigin {
    Left,
    Center,
    Right,
}

impl FromStr for RelativeXOrigin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(RelativeXOrigin::Left),
            "center" => Ok(RelativeXOrigin::Center),
            "right" => Ok(RelativeXOrigin::Right),
            _ => Err(()),
        }
    }
}

impl fmt::Display for RelativeXOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RelativeXOrigin::*;
        match self {
            Left => write!(f, "left"),
            Center => write!(f, "center"),
            Right => write!(f, "right"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum RelativeYOrigin {
    Top,
    Center,
    Bottom,
}

impl FromStr for RelativeYOrigin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(RelativeYOrigin::Top),
            "center" => Ok(RelativeYOrigin::Center),
            "bottom" => Ok(RelativeYOrigin::Bottom),
            _ => Err(()),
        }
    }
}

impl fmt::Display for RelativeYOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RelativeYOrigin::*;
        match self {
            Top => write!(f, "top"),
            Center => write!(f, "center"),
            Bottom => write!(f, "bottom"),
        }
    }
}
