#[cfg(feature = "serde")]
use serde::Serialize;
use url::Url;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ImageSource(Url);
