use crate::unit::Percent;

#[derive(Clone, Debug, PartialEq)]
pub enum LineHeight<Unit = f64> {
    Absolute(Unit),
    /// A percent
    Relative(Percent),
}
impl<Unit> LineHeight<Unit> {
    pub fn convert<NewUnit>(self) -> LineHeight<NewUnit>
    where
        Unit: Into<NewUnit>,
    {
        match self {
            Self::Absolute(u) => LineHeight::Absolute(u.into()),
            Self::Relative(u) => LineHeight::Relative(u),
        }
    }

    pub fn absolute(unit: impl Into<Unit>) -> Self {
        LineHeight::Absolute(unit.into())
    }

    pub fn percent(percent: impl Into<Percent>) -> Self {
        LineHeight::Relative(percent.into())
    }

    pub fn relative(percent: impl Into<Percent>) -> Self {
        Self::percent(percent)
    }
}

impl<Unit> From<Percent> for LineHeight<Unit> {
    fn from(value: Percent) -> Self {
        Self::percent(value)
    }
}
